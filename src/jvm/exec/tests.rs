use super::*;

#[test]
fn monitors_are_reentrant_and_identity_based() {
    let jvm = create_runtime_const();
    let lock = RuntimeValue::String(rc::Rc::new("lock".to_string()));
    let other = RuntimeValue::String(rc::Rc::new("lock".to_string()));
    execute_monitor(&jvm, lock.clone(), true).unwrap();
    execute_monitor(&jvm, lock.clone(), true).unwrap();
    assert_eq!(jvm.monitors.borrow()[0].1, 2);
    execute_monitor(&jvm, other, false).unwrap();
    let exception = jvm.pending_exception.borrow_mut().take().unwrap();
    assert!(reference_assignable(&jvm, &exception, "java/lang/IllegalMonitorStateException"));
    assert_eq!(jvm.monitors.borrow()[0].1, 2);
    execute_monitor(&jvm, lock.clone(), false).unwrap();
    assert_eq!(jvm.monitors.borrow()[0].1, 1);
    execute_monitor(&jvm, lock.clone(), false).unwrap();
    assert!(jvm.monitors.borrow().is_empty());
    execute_monitor(&jvm, lock, false).unwrap();
    assert!(jvm.pending_exception.borrow().is_some());
}

#[test]
fn monitors_reject_null_and_nonreferences() {
    let jvm = create_runtime_const();
    for enter in [true, false] {
        execute_monitor(&jvm, RuntimeValue::Null, enter).unwrap();
        let exception = jvm.pending_exception.borrow_mut().take().unwrap();
        assert!(reference_assignable(&jvm, &exception, "java/lang/NullPointerException"));
        assert!(execute_monitor(&jvm, RuntimeValue::Int(1), enter).is_err());
        assert!(jvm.monitors.borrow().is_empty());
    }
}

#[test]
fn lookup_switch_alignment_signed_keys_and_offsets() {
    for pc in 4..8 {
        let start = (pc + 4) & !3;
        let mut code = vec![0; start];
        code[pc] = opcodes::LOOKUPSWITCH;
        for value in [-4_i32, 2, -1000, 40, 1000, -4] {
            code.extend_from_slice(&value.to_be_bytes());
        }
        code.resize(64, 0);
        assert_eq!(lookup_switch_target(&code, pc, -1000).unwrap(), pc + 40);
        assert_eq!(lookup_switch_target(&code, pc, 1000).unwrap(), pc - 4);
        assert_eq!(lookup_switch_target(&code, pc, 0).unwrap(), pc - 4);
    }
}

#[test]
fn lookup_switch_empty_and_malformed_tables() {
    let mut code = vec![opcodes::LOOKUPSWITCH, 0, 0, 0];
    code.extend_from_slice(&12_i32.to_be_bytes());
    code.extend_from_slice(&0_i32.to_be_bytes());
    code.push(opcodes::RETURN);
    assert_eq!(lookup_switch_target(&code, 0, 42).unwrap(), 12);
    assert!(lookup_switch_target(&code[..8], 0, 0).is_err());
    code[8..12].copy_from_slice(&1_i32.to_be_bytes());
    assert!(lookup_switch_target(&code, 0, 0).is_err());
    code[8..12].copy_from_slice(&(-1_i32).to_be_bytes());
    assert!(lookup_switch_target(&code, 0, 0).is_err());
    code[8..12].copy_from_slice(&0_i32.to_be_bytes());
    code[4..8].copy_from_slice(&(-1_i32).to_be_bytes());
    assert!(lookup_switch_target(&code, 0, 0).is_err());
}

#[test]
fn wide_integer_locals_and_wrapping_increment() {
    let mut frame = Frame {
        stack: vec![RuntimeValue::Int(i32::MAX as i64)],
        locals: vec![RuntimeValue::Void; 257],
    };
    let store = [opcodes::WIDE, opcodes::ISTORE, 1, 0];
    assert_eq!(execute_wide(&store, 0, &mut frame).unwrap(), 4);
    let increment = [opcodes::WIDE, opcodes::IINC, 1, 0, 0, 1];
    assert_eq!(execute_wide(&increment, 0, &mut frame).unwrap(), 6);
    let load = [opcodes::WIDE, opcodes::ILOAD, 1, 0];
    assert_eq!(execute_wide(&load, 0, &mut frame).unwrap(), 4);
    assert!(matches!(frame.stack.as_slice(), [RuntimeValue::Int(value)] if *value == i32::MIN as i64));
}

#[test]
fn wide_rejects_truncation_bad_indexes_and_types() {
    for code in [
        vec![opcodes::WIDE],
        vec![opcodes::WIDE, opcodes::IINC, 0, 0, 0],
        vec![opcodes::WIDE, opcodes::ILOAD, 1, 0],
        vec![opcodes::WIDE, opcodes::ISTORE, 1, 0],
        vec![opcodes::WIDE, opcodes::ILOAD, 0, 0],
        vec![opcodes::WIDE, opcodes::ISTORE, 0, 0],
        vec![opcodes::WIDE, opcodes::IINC, 0, 0, 0, 1],
        vec![opcodes::WIDE, opcodes::NOP, 0, 0],
    ] {
        let mut frame = Frame {
            stack: vec![RuntimeValue::Null],
            locals: vec![RuntimeValue::Null],
        };
        assert!(execute_wide(&code, 0, &mut frame).is_err());
    }
}

#[test]
fn swap_category_one_values() {
    use RuntimeValue::{Int as I, Null};
    for (stack, expected) in [
        (vec![I(3), I(1), I(2)], vec![I(3), I(2), I(1)]),
        (vec![I(3), Null, I(1)], vec![I(3), I(1), Null]),
    ] {
        let mut frame = Frame { stack, locals: Vec::new() };
        swap_values(&mut frame).unwrap();
        assert_eq!(format!("{:?}", frame.stack), format!("{:?}", expected));
    }
}

#[test]
fn swap_rejects_invalid_stacks_without_mutating_them() {
    use RuntimeValue::{Double as D, Int as I, Long as L, Void};
    for stack in [
        vec![], vec![I(1)], vec![I(1), L(2)], vec![L(1), I(2)],
        vec![I(1), D(2.0)], vec![D(1.0), I(2)],
        vec![I(1), Void], vec![Void, I(1)],
    ] {
        let before = format!("{:?}", stack);
        let mut frame = Frame { stack, locals: Vec::new() };
        assert!(swap_values(&mut frame).is_err());
        assert_eq!(format!("{:?}", frame.stack), before);
    }
}

#[test]
fn pop_valid_forms() {
    use RuntimeValue::{Double as D, Int as I, Long as L, Null};
    for (slots, stack) in [
        (1, vec![I(99), I(1)]),
        (1, vec![I(99), Null]),
        (2, vec![I(99), I(1), I(2)]),
        (2, vec![I(99), L(1)]),
        (2, vec![I(99), D(1.0)]),
    ] {
        let mut frame = Frame { stack, locals: Vec::new() };
        pop_slots(&mut frame, slots).unwrap();
        assert!(matches!(frame.stack.as_slice(), [RuntimeValue::Int(99)]));
    }
}

#[test]
fn pop_rejects_invalid_stacks_without_mutating_them() {
    use RuntimeValue::{Double as D, Int as I, Long as L, Void};
    for (slots, stack) in [
        (1, vec![]),
        (2, vec![]),
        (1, vec![L(1)]),
        (1, vec![D(1.0)]),
        (2, vec![I(1)]),
        (2, vec![L(1), I(2)]),
        (1, vec![Void]),
        (2, vec![Void, I(1)]),
    ] {
        let before = format!("{:?}", stack);
        let mut frame = Frame { stack, locals: Vec::new() };
        assert!(pop_slots(&mut frame, slots).is_err());
        assert_eq!(format!("{:?}", frame.stack), before);
    }
}

#[test]
fn dup2_x_forms() {
    use RuntimeValue::{Double as D, Int as I, Long as L};
    let cases = vec![
        (1, vec![I(1), I(2), I(3)], vec![I(2), I(3), I(1), I(2), I(3)]),
        (1, vec![I(1), D(2.0)], vec![D(2.0), I(1), D(2.0)]),
        (2, vec![I(1), I(2), I(3), I(4)], vec![I(3), I(4), I(1), I(2), I(3), I(4)]),
        (2, vec![L(1), I(2), I(3)], vec![I(2), I(3), L(1), I(2), I(3)]),
        (2, vec![I(1), I(2), D(3.0)], vec![D(3.0), I(1), I(2), D(3.0)]),
        (2, vec![L(1), D(2.0)], vec![D(2.0), L(1), D(2.0)]),
    ];
    for (depth, mut stack, mut expected) in cases {
        // Ensure values below the affected operands are preserved.
        stack.insert(0, I(99));
        expected.insert(0, I(99));
        let mut frame = Frame { stack, locals: Vec::new() };
        duplicate_two_slots(&mut frame, depth).unwrap();
        assert_eq!(format!("{:?}", frame.stack), format!("{:?}", expected));
    }
}

#[test]
fn dup2_x_rejects_invalid_stacks_without_mutating_them() {
    use RuntimeValue::{Double as D, Int as I, Void};
    let cases = vec![
        (1, vec![]),
        (1, vec![I(1)]),
        (1, vec![D(1.0), I(2)]),
        (1, vec![D(1.0), D(2.0)]),
        (2, vec![I(1), D(2.0)]),
        (2, vec![D(1.0), I(2), D(3.0)]),
        (1, vec![I(1), Void]),
    ];
    for (depth, stack) in cases {
        let before = format!("{:?}", stack);
        let mut frame = Frame { stack, locals: Vec::new() };
        assert!(duplicate_two_slots(&mut frame, depth).is_err());
        assert_eq!(format!("{:?}", frame.stack), before);
    }
}

use super::*;

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

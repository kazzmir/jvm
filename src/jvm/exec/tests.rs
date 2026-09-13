use super::*;

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

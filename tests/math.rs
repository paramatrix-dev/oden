use anvil::{Cuboid, IntoLength};
use oden::{Member, compile_input, eval_str};

#[test]
fn test_addition_before_multiplication() {
    assert_eq!(eval_str("2 + 3 * 4"), Ok(Member::Instance(Box::new(14.))));
}

#[test]
fn test_multiplication_before_addition() {
    assert_eq!(eval_str("2 * 3 + 4"), Ok(Member::Instance(Box::new(10.))));
}

#[test]
fn test_double_addition() {
    assert_eq!(eval_str("1 + 2 + 3"), Ok(Member::Instance(Box::new(6.))));
}

#[test]
fn test_multiplication_with_brackets() {
    assert_eq!(eval_str("(1 + 2) * 3"), Ok(Member::Instance(Box::new(9.))));
}

#[test]
fn test_negative_number() {
    assert_eq!(eval_str("-2"), Ok(Member::Instance(Box::new(-2.))));
    assert_eq!(eval_str("-2mm"), Ok(Member::Instance(Box::new(-2.mm()))));
}

#[test]
fn test_math_in_part_construction() {
    let text = "
        part Box:
            factor = 5
            size = 1mm * factor
            part.add(Cube(size))
        ";
    assert_eq!(compile_input(text), Ok(Cuboid::from_mm(5., 5., 5.)))
}

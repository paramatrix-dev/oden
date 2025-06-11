use oden::{Error, Span, compile_input};

#[test]
fn test_call_from_str_second_argument_missing() {
    let text = "
    part Box:
        part.(Cuboid(5mm, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedExpression(Span(23, 51, "".into())))
    )
}

#[test]
fn test_call_from_str_only_dot() {
    let text = "
    part Box:
        part.
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedExpression(Span(23, 28, "".into())))
    )
}

#[test]
fn test_missing_arguments() {
    let text = "
    part Box:
        part.add(Cuboid(5mm, 6mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Length".into(), "Length".into(), "Length".into()],
            is: vec!["Length".into(), "Length".into()],
            span: Span(32, 48, "".into())
        })
    )
}

#[test]
fn test_too_many_arguments() {
    let text = "
    part Box:
        part.add(Cuboid(5mm, 6mm, 7mm, 8mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Length".into(), "Length".into(), "Length".into()],
            is: vec![
                "Length".into(),
                "Length".into(),
                "Length".into(),
                "Length".into()
            ],
            span: Span(32, 58, "".into())
        })
    )
}

#[test]
fn test_wrong_argument_type() {
    let text = "
    part Box:
        part.add(Cuboid(5mm, 6mm, Cuboid(7mm, 8mm, 9mm)))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Length".into(), "Length".into(), "Length".into()],
            is: vec!["Length".into(), "Length".into(), "Part".into(),],
            span: Span(32, 71, "".into())
        })
    )
}

#[test]
fn test_undefined_variable() {
    let text = "
    part Box:
        lala.add(Cuboid(5mm, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownVariable(
            "lala".into(),
            Span(23, 27, "".into())
        ))
    )
}

#[test]
fn test_unknown_method() {
    let text = "
    part Box:
        part.unknown(Cuboid(1mm, 2mm, 3mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownMethod(
            "unknown".into(),
            Span(23, 58, "".into())
        ))
    )
}

#[test]
fn test_unknown_function() {
    let text = "
    part Box:
        part.add(Unknown(1mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownFunction(
            "Unknown".into(),
            Span(32, 44, "".into())
        ))
    )
}

#[test]
fn test_part_name_missing() {
    let text = "
    part :
        part.add(Cuboid(5mm, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedIdentifyer(Span(5, 11, "".into())))
    )
}

#[test]
fn test_first_part_of_assignment_missing() {
    let text = "
    part Box:
        = 10mm
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedIdentifyer(Span(23, 29, "".into())))
    )
}

#[test]
fn test_second_part_of_assignment_missing() {
    let text = "
    part Box:
        height =
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedExpression(Span(23, 31, "".into())))
    )
}

#[test]
fn test_double_period() {
    let text = "
    part Box:
        part..add(Cuboid(5mm, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::ExpectedExpression(Span(23, 28, "".into())))
    )
}

#[test]
fn test_scalar_instead_of_length() {
    let text = "
    part Box:
        part.add(Cuboid(5, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Length".into(), "Length".into(), "Length".into()],
            is: vec!["Number".into(), "Length".into(), "Length".into(),],
            span: Span(32, 51, "".into())
        })
    )
}

#[test]
fn test_unknown_length_unit() {
    let text = "
    part Box:
        part.add(Cuboid(5meter, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownUnit("meter".into(), Span(39, 45, "".into())))
    )
}

#[test]
fn test_too_many_args_in_add() {
    let text = "
    part Box:
        part.add(Cuboid(5mm, 6mm, 7mm), 5mm)
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Part".into()],
            is: vec!["Part".into(), "Length".into(),],
            span: Span(23, 59, "".into())
        })
    )
}

#[test]
fn test_add_wrong_argument_type() {
    let text = "
    part Box:
        part.add(5mm)
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::Arguments {
            should: vec!["Part".into()],
            is: vec!["Length".into(),],
            span: Span(23, 36, "".into())
        })
    )
}

#[test]
fn test_undefined_reference() {
    let text = "
    part Box:
        part.add(Cuboid(height, 6mm, 7mm))
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownVariable(
            "height".into(),
            Span(39, 45, "".into())
        ))
    )
}

#[test]
fn test_method_call_on_length() {
    let text = "
    part Box:
        height = 5mm
        height.move_to(5mm, 6mm, 7mm)
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownMethod(
            "move_to".into(),
            Span(44, 73, "".into())
        ))
    )
}

#[test]
fn test_method_call_on_num() {
    let text = "
    part Box:
        num = 10
        num.move_to(5mm, 6mm, 7mm)
    ";

    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::UnknownMethod(
            "move_to".into(),
            Span(40, 66, "".into())
        ))
    )
}

#[test]
fn test_empty_sketch() {
    let text = "
        part Box:
            sketch = Rectangle(5mm, 6mm)
            part.add(sketch.extrude(Plane.XY(), 0mm))
        ";
    assert_eq!(
        compile_input(text, "".into()),
        Err(Error::EmptyPart(Span(81, 112, "".into())))
    )
}

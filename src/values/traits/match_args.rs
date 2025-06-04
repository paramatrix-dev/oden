use anvil::{Angle, Axis, Length, Part, Plane, Sketch};

use crate::{Error, Span, Value};

pub fn match_empty_args(args: &[Value], span: Span) -> Result<(), Error> {
    match args {
        [] => Ok(()),
        _ => Err(Error::ArgumentNumber {
            should: 0,
            is: args.len(),
            span,
        }),
    }
}

pub fn match_angle_arg(args: &[Value], span: Span) -> Result<Angle, Error> {
    match args {
        [Value::Angle(first)] => Ok(*first),
        _ => Err(check_args(args, vec!["Angle"], span)),
    }
}

pub fn match_length_arg(args: &[Value], span: Span) -> Result<Length, Error> {
    match args {
        [Value::Length(first)] => Ok(*first),
        _ => Err(check_args(args, vec!["Length"], span)),
    }
}

pub fn match_two_length_args(args: &[Value], span: Span) -> Result<(Length, Length), Error> {
    match args {
        [Value::Length(first), Value::Length(second)] => Ok((*first, *second)),
        _ => Err(check_args(args, vec!["Length", "Length"], span)),
    }
}

pub fn match_three_length_args(
    args: &[Value],
    span: Span,
) -> Result<(Length, Length, Length), Error> {
    match args {
        [
            Value::Length(first),
            Value::Length(second),
            Value::Length(third),
        ] => Ok((*first, *second, *third)),
        _ => Err(check_args(args, vec!["Length", "Length", "Length"], span)),
    }
}

pub fn match_num_arg(args: &[Value], span: Span) -> Result<f64, Error> {
    match args {
        [Value::Number(first)] => Ok(*first),
        _ => Err(check_args(args, vec!["Number"], span)),
    }
}

pub fn match_part_arg(args: &[Value], span: Span) -> Result<&Part, Error> {
    match args {
        [Value::Part(first)] => Ok(first),
        _ => Err(check_args(args, vec!["Part"], span)),
    }
}

pub fn match_sketch_arg(args: &[Value], span: Span) -> Result<&Sketch, Error> {
    match args {
        [Value::Sketch(first)] => Ok(first),
        _ => Err(check_args(args, vec!["Sketch"], span)),
    }
}

pub fn match_axis_angle_arg(args: &[Value], span: Span) -> Result<(Axis<3>, Angle), Error> {
    match args {
        [Value::Axis(first), Value::Angle(second)] => Ok((*first, *second)),
        _ => Err(check_args(args, vec!["Axis", "Angle"], span)),
    }
}

pub fn match_axis_num_arg(args: &[Value], span: Span) -> Result<(Axis<3>, f64), Error> {
    match args {
        [Value::Axis(first), Value::Number(second)] => Ok((*first, *second)),
        _ => Err(check_args(args, vec!["Axis", "Number"], span)),
    }
}

pub fn match_plane_length_arg(args: &[Value], span: Span) -> Result<(Plane, Length), Error> {
    match args {
        [Value::Plane(first), Value::Length(second)] => Ok((*first, *second)),
        _ => Err(check_args(args, vec!["Plane", "Length"], span)),
    }
}

fn check_args(args: &[Value], should: Vec<&str>, span: Span) -> Error {
    if args.len() != should.len() {
        return Error::ArgumentNumber {
            should: should.len(),
            is: args.len(),
            span,
        };
    }
    for (i, arg) in args.iter().enumerate() {
        if arg.type_str() != *should[i] {
            return Error::ArgumentType {
                should: should[i].to_string(),
                span,
            };
        }
    }

    panic!("check args should always find an error")
}

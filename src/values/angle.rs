use anvil::Angle;

use crate::{
    Error, Instance, Span, Value,
    values::traits::match_args::{match_angle_arg, match_num_arg},
};

impl Instance for Angle {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => Ok(Value::Angle(*self + match_angle_arg(args, span)?)),
            "subtract" => Ok(Value::Angle(*self - match_angle_arg(args, span)?)),
            "multiply" => Ok(Value::Angle(*self * match_num_arg(args, span)?)),
            "divide" => Ok(Value::Angle(*self / match_num_arg(args, span)?)),
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Angle".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoAngle;

    use crate::eval_str;

    use super::*;

    #[test]
    fn instance_method_add() {
        let actual = eval_str("1rad + 2rad");
        assert_eq!(actual, Ok(Value::Angle(3.rad())))
    }

    #[test]
    fn instance_method_subtract() {
        let actual = eval_str("3rad - 2rad");
        assert_eq!(actual, Ok(Value::Angle(1.rad())))
    }

    #[test]
    fn instance_method_multiply() {
        let actual = eval_str("2rad * 3");
        assert_eq!(actual, Ok(Value::Angle(6.rad())))
    }

    #[test]
    fn instance_method_divide() {
        let actual = eval_str("2rad / 2");
        assert_eq!(actual, Ok(Value::Angle(1.rad())))
    }
}

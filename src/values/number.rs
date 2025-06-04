use crate::{Error, Instance, Span, Value, values::traits::match_args::match_num_arg};

impl Instance for f64 {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => Ok(Value::Number(*self + match_num_arg(args, span)?)),
            "subtract" => Ok(Value::Number(*self - match_num_arg(args, span)?)),
            "multiply" => match args {
                [Value::Angle(other)] => Ok(Value::Angle(*self * *other)),
                [Value::Length(other)] => Ok(Value::Length(*self * *other)),
                [Value::Number(other)] => Ok(Value::Number(*self * other)),
                [_] => Err(Error::ArgumentType {
                    should: "Angle or Length or Number".into(),
                    span,
                }),
                _ => Err(Error::ArgumentNumber {
                    should: 1,
                    is: args.len(),
                    span,
                }),
            },
            "divide" => Ok(Value::Number(*self / match_num_arg(args, span)?)),
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Number".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;

    use super::*;

    #[test]
    fn instance_method_add() {
        let actual = eval_str("1 + 2");
        assert_eq!(actual, Ok(Value::Number(3.)))
    }

    #[test]
    fn instance_method_subtract() {
        let actual = eval_str("3 - 2");
        assert_eq!(actual, Ok(Value::Number(1.)))
    }

    #[test]
    fn instance_method_multiply() {
        let actual = eval_str("2 * 3");
        assert_eq!(actual, Ok(Value::Number(6.)))
    }

    #[test]
    fn instance_method_divide() {
        let actual = eval_str("2 / 2");
        assert_eq!(actual, Ok(Value::Number(1.)))
    }
}

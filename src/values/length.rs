use anvil::Length;

use crate::{
    Error, Instance, Span, Value,
    values::traits::match_args::{match_length_arg, match_num_arg},
};

impl Instance for Length {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => Ok(Value::Length(*self + match_length_arg(args, span)?)),
            "subtract" => Ok(Value::Length(*self - match_length_arg(args, span)?)),
            "multiply" => Ok(Value::Length(*self * match_num_arg(args, span)?)),
            "divide" => Ok(Value::Length(*self / match_num_arg(args, span)?)),
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Length".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use crate::eval_str;

    use super::*;

    #[test]
    fn instance_method_add() {
        let actual = eval_str("1m + 2m");
        assert_eq!(actual, Ok(Value::Length(3.m())))
    }

    #[test]
    fn instance_method_subtract() {
        let actual = eval_str("3m - 2m");
        assert_eq!(actual, Ok(Value::Length(1.m())))
    }

    #[test]
    fn instance_method_multiply() {
        let actual = eval_str("2m * 3");
        assert_eq!(actual, Ok(Value::Length(6.m())))
    }

    #[test]
    fn instance_method_divide() {
        let actual = eval_str("2m / 2");
        assert_eq!(actual, Ok(Value::Length(1.m())))
    }
}

use anvil::Length;

use crate::{Error, Span, Instance, Value, check_args};

impl Instance for Length {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => {
                check_args(args, vec!["Length"], span)?;
                match args {
                    [Value::Length(other)] => Ok(Value::Length(*self + *other)),
                    _ => unreachable!(),
                }
            }
            "subtract" => {
                check_args(args, vec!["Length"], span)?;
                match args {
                    [Value::Length(other)] => Ok(Value::Length(*self - *other)),
                    _ => unreachable!(),
                }
            }
            "multiply" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Length(*self * *other)),
                    _ => unreachable!(),
                }
            }
            "divide" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Length(*self / *other)),
                    _ => unreachable!(),
                }
            }
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Length".into()
    }
}

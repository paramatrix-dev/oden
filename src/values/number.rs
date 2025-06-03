use crate::{errors::Error, syntax::Span};

use super::{
    Value,
    inner_value::{TypeInstance, check_args},
};

impl TypeInstance for f64 {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Number(*self + *other)),
                    _ => unreachable!(),
                }
            }
            "subtract" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Number(*self - *other)),
                    _ => unreachable!(),
                }
            }
            "multiply" => {
                match check_args(args, vec!["Number"], span.clone()) {
                    Ok(()) => (),
                    Err(_) => check_args(args, vec!["Length"], span)?,
                }
                match args {
                    [Value::Number(other)] => Ok(Value::Number(*self * *other)),
                    [Value::Length(other)] => Ok(Value::Length(*self * *other)),
                    _ => unreachable!(),
                }
            }
            "divide" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Number(*self / *other)),
                    _ => unreachable!(),
                }
            }
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Number".into()
    }
}

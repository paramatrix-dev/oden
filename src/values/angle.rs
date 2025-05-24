use anvil::Angle;

use crate::{errors::Error, syntax::Span};

use super::{
    Value,
    inner_value::{InnerValue, check_args},
};

impl InnerValue for Angle {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => {
                check_args(args, vec!["Angle"], span)?;
                match args {
                    [Value::Angle(other)] => Ok(Value::Angle(*self + *other)),
                    _ => unreachable!(),
                }
            }
            "subtract" => {
                check_args(args, vec!["Angle"], span)?;
                match args {
                    [Value::Angle(other)] => Ok(Value::Angle(*self - *other)),
                    _ => unreachable!(),
                }
            }
            "multiply" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Angle(*self * *other)),
                    _ => unreachable!(),
                }
            }
            "divide" => {
                check_args(args, vec!["Number"], span)?;
                match args {
                    [Value::Number(other)] => Ok(Value::Angle(*self / *other)),
                    _ => unreachable!(),
                }
            }
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Angle".into()
    }
}

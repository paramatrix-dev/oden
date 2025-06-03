use anvil::{Path, Point};

use crate::{errors::Error, syntax::Span};

use super::{check_args, InnerValue, Value};

impl InnerValue for Path {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "line_to" => {
                check_args(args, vec!["Length", "Length"], span)?;
                match args {
                    [Value::Length(x), Value::Length(y)] => {
                        Ok(Value::Path(self.line_to(Point::<2>::new([*x, *y]))))
                    }
                    _ => unreachable!(),
                }
            }
            "close" => {
                check_args(args, vec![], span)?;
                Ok(Value::Sketch(self.clone().close()))
            }
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Path".into()
    }
}

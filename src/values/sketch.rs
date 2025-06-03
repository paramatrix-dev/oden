use anvil::{Sketch, point};

use crate::{Error, Span, Instance, Value, check_args};

impl Instance for Sketch {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => {
                check_args(args, vec!["Sketch"], span)?;
                match args {
                    [Value::Sketch(other)] => Ok(Value::Sketch(self.add(other))),
                    _ => unreachable!(),
                }
            }
            "extrude" => {
                check_args(args, vec!["Plane", "Length"], span.clone())?;
                match args {
                    [Value::Plane(plane), Value::Length(thickness)] => Ok(Value::Part(
                        Error::from_anvil(self.extrude(*plane, *thickness), Some(span))?,
                    )),
                    _ => unreachable!(),
                }
            }
            "intersect" => {
                check_args(args, vec!["Sketch"], span)?;
                match args {
                    [Value::Sketch(other)] => Ok(Value::Sketch(self.intersect(other))),
                    _ => unreachable!(),
                }
            }
            "move_to" => {
                check_args(args, vec!["Length", "Length"], span)?;
                match args {
                    [Value::Length(x), Value::Length(y)] => {
                        Ok(Value::Sketch(self.move_to(point!(*x, *y))))
                    }
                    _ => unreachable!(),
                }
            }
            "rotate" => {
                check_args(args, vec!["Angle"], span)?;
                match args {
                    [Value::Angle(angle)] => Ok(Value::Sketch(self.rotate(*angle))),
                    _ => unreachable!(),
                }
            }
            "subtract" => {
                check_args(args, vec!["Sketch"], span)?;
                match args {
                    [Value::Sketch(other)] => Ok(Value::Sketch(self.subtract(other))),
                    _ => unreachable!(),
                }
            }
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Sketch".into()
    }
}

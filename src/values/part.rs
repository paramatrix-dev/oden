use anvil::{Part, Point3D};

use crate::{errors::Error, syntax::Span};

use super::{
    Value,
    inner_value::{InnerValue, check_args},
};

impl InnerValue for Part {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => {
                check_args(args, vec!["Part"], span)?;
                match args {
                    [Value::Part(other)] => Ok(Value::Part(self.add(other))),
                    _ => unreachable!(),
                }
            }
            "circular_pattern" => {
                check_args(args, vec!["Axis", "Number"], span)?;
                match args {
                    [Value::Axis(axis), Value::Number(n)] => {
                        Ok(Value::Part(self.circular_pattern(axis.clone(), *n as u8)))
                    }
                    _ => unreachable!(),
                }
            }
            "intersect" => {
                check_args(args, vec!["Part"], span)?;
                match args {
                    [Value::Part(other)] => Ok(Value::Part(self.intersect(other))),
                    _ => unreachable!(),
                }
            }
            "move_to" => {
                check_args(args, vec!["Length", "Length", "Length"], span)?;
                match args {
                    [Value::Length(x), Value::Length(y), Value::Length(z)] => {
                        Ok(Value::Part(self.move_to(Point3D::new(*x, *y, *z))))
                    }
                    _ => unreachable!(),
                }
            }
            "rotate_around" => {
                check_args(args, vec!["Axis", "Angle"], span)?;
                match args {
                    [Value::Axis(axis), Value::Angle(angle)] => {
                        Ok(Value::Part(self.rotate_around(axis.clone(), *angle)))
                    }
                    _ => unreachable!(),
                }
            }
            "subtract" => {
                check_args(args, vec!["Part"], span)?;
                match args {
                    [Value::Part(other)] => Ok(Value::Part(self.subtract(other))),
                    _ => unreachable!(),
                }
            }
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Part".into()
    }
}

use anvil::{Path, point};

use crate::{
    Error, Instance, Span, Type, Value,
    values::traits::match_args::{match_empty_args, match_two_length_args},
};

#[derive(Clone, Debug, PartialEq)]
pub struct PathType;
impl Type for PathType {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        let (x, y) = match_two_length_args(args, span)?;
        Ok(Value::Path(Path::at(point!(x, y))))
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Path".into()
    }
}
impl Instance for PathType {
    fn type_str(&self) -> String {
        "Type".into()
    }
}

impl Instance for Path {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "close" => {
                match_empty_args(args, span)?;
                Ok(Value::Sketch(self.clone().close()))
            }
            "line_to" => {
                let (x, y) = match_two_length_args(args, span)?;
                Ok(Value::Path(self.line_to(point!(x, y))))
            }
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Path".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use crate::eval_str;

    use super::*;

    #[test]
    fn call() {
        let actual = eval_str("Path(1m, 2m)");
        assert_eq!(actual, Ok(Value::Path(Path::at(point!(1.m(), 2.m())))))
    }

    #[test]
    fn instance_method_close() {
        let actual = eval_str("Path(1m, 2m).close()");
        assert_eq!(
            actual,
            Ok(Value::Sketch(Path::at(point!(1.m(), 2.m())).close()))
        )
    }

    #[test]
    fn instance_method_line_to() {
        let actual = eval_str("Path(1m, 2m).line_to(3m, 4m)");
        assert_eq!(
            actual,
            Ok(Value::Path(
                Path::at(point!(1.m(), 2.m())).line_to(point!(3.m(), 4.m()))
            ))
        )
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Path.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 14, "".into())
            ))
        )
    }
}

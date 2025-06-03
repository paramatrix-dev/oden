use anvil::Cube;

use crate::{Error, Instance, Span, Type, Value, check_args};

impl Type for Cube {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length"], span)?;
        match args {
            [Value::Length(size)] => Ok(Value::Part(Cube::from_size(*size))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cube".into()
    }
}
impl Instance for Cube {
    fn type_str(&self) -> String {
        "Type".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use crate::eval_str;

    use super::*;

    #[test]
    fn call() {
        let actual = eval_str("Cube(1m)");
        assert_eq!(actual, Ok(Value::Part(Cube::from_size(1.m()))))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Cube.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 14, "".into())
            ))
        )
    }
}

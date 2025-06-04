use anvil::Cube;

use crate::{Error, Instance, Span, Type, Value, values::traits::match_args::match_length_arg};

impl Type for Cube {
    fn name(&self) -> String {
        "Cube".into()
    }
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        Ok(Value::Part(Cube::from_size(match_length_arg(args, span)?)))
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
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

use anvil::Circle;

use crate::{Error, Instance, Span, Type, Value, check_args};

impl Type for Circle {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length"], span)?;
        match args {
            [Value::Length(radius)] => Ok(Value::Sketch(Circle::from_radius(*radius))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Circle".into()
    }
}
impl Instance for Circle {
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
        let actual = eval_str("Circle(1m)");
        assert_eq!(actual, Ok(Value::Sketch(Circle::from_radius(1.m()))))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Circle.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 16, "".into())
            ))
        )
    }
}

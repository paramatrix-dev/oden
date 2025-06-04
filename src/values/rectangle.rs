use anvil::Rectangle;

use crate::{
    Error, Instance, Span, Type, Value, values::traits::match_args::match_two_length_args,
};

impl Type for Rectangle {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        let (x, y) = match_two_length_args(args, span)?;
        Ok(Value::Sketch(Rectangle::from_dim(x, y)))
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Rectangle".into()
    }
}
impl Instance for Rectangle {
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
        let actual = eval_str("Rectangle(1m, 2m)");
        assert_eq!(actual, Ok(Value::Sketch(Rectangle::from_dim(1.m(), 2.m()))))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Rectangle.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 19, "".into())
            ))
        )
    }
}

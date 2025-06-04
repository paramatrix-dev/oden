use anvil::Cylinder;

use crate::{
    Error, Instance, Span, Type, Value, values::traits::match_args::match_two_length_args,
};

impl Type for Cylinder {
    fn name(&self) -> String {
        "Cylinder".into()
    }
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        let (radius, height) = match_two_length_args(args, span)?;
        Ok(Value::Part(Cylinder::from_radius(radius, height)))
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
}
impl Instance for Cylinder {
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
        let actual = eval_str("Cylinder(1m, 2m)");
        assert_eq!(actual, Ok(Value::Part(Cylinder::from_radius(1.m(), 2.m()))))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Cylinder.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 18, "".into())
            ))
        )
    }
}

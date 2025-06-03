use anvil::Sphere;

use crate::{Error, Instance, Span, Type, Value, check_args};

impl Type for Sphere {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length"], span)?;
        match args {
            [Value::Length(radius)] => Ok(Value::Part(Sphere::from_radius(*radius))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Sphere".into()
    }
}
impl Instance for Sphere {
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
        let actual = eval_str("Sphere(1m)");
        assert_eq!(actual, Ok(Value::Part(Sphere::from_radius(1.m()))))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Sphere.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 16, "".into())
            ))
        )
    }
}

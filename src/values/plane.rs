use anvil::Plane;

use crate::{Error, Instance, Span, Type, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct PlaneType;
impl Type for PlaneType {
    fn name(&self) -> String {
        "Plane".into()
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
}
impl Instance for PlaneType {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "XY" => Ok(Value::Plane(Plane::xy())),
            "XZ" => Ok(Value::Plane(Plane::xz())),
            "YZ" => Ok(Value::Plane(Plane::yz())),
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

impl Instance for Plane {
    fn type_str(&self) -> String {
        "Plane".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;

    use super::*;

    #[test]
    fn call() {
        let actual = eval_str("Plane()");
        assert_eq!(
            actual,
            Err(Error::NotCallable("Plane".into(), Span(0, 7, "".into())))
        )
    }

    #[test]
    fn type_method_xy() {
        let actual = eval_str("Plane.XY()");
        assert_eq!(actual, Ok(Value::Plane(Plane::xy())))
    }

    #[test]
    fn type_method_xz() {
        let actual = eval_str("Plane.XZ()");
        assert_eq!(actual, Ok(Value::Plane(Plane::xz())))
    }

    #[test]
    fn type_method_yz() {
        let actual = eval_str("Plane.YZ()");
        assert_eq!(actual, Ok(Value::Plane(Plane::yz())))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Plane.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 15, "".into())
            ))
        )
    }
}

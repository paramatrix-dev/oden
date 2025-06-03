use anvil::Axis;

use crate::{Error, Instance, Span, Type, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct AxisType;
impl Type for AxisType {
    fn name(&self) -> String {
        "Axis".into()
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
}
impl Instance for AxisType {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "X" => Ok(Value::Axis(Axis::<3>::x())),
            "Y" => Ok(Value::Axis(Axis::<3>::y())),
            "Z" => Ok(Value::Axis(Axis::<3>::z())),
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

impl Instance for Axis<3> {
    fn type_str(&self) -> String {
        AxisType.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;

    use super::*;

    #[test]
    fn call() {
        let actual = eval_str("Axis()");
        assert_eq!(
            actual,
            Err(Error::NotCallable("Axis".into(), Span(0, 6, "".into())))
        )
    }

    #[test]
    fn type_method_x() {
        let actual = eval_str("Axis.X()");
        assert_eq!(actual, Ok(Value::Axis(Axis::<3>::x())))
    }

    #[test]
    fn type_method_y() {
        let actual = eval_str("Axis.Y()");
        assert_eq!(actual, Ok(Value::Axis(Axis::<3>::y())))
    }

    #[test]
    fn type_method_z() {
        let actual = eval_str("Axis.Z()");
        assert_eq!(actual, Ok(Value::Axis(Axis::<3>::z())))
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Axis.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 14, "".into())
            ))
        )
    }
}

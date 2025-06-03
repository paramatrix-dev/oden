use anvil::Axis;

use crate::{Error, Span, Instance, Value};

impl Instance for Axis<3> {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Axis".into()
    }
}

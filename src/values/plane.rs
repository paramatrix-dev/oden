use anvil::Plane;

use crate::{Error, Span, Instance, Value};

impl Instance for Plane {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Plane".into()
    }
}

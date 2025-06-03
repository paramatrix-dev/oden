use anvil::Plane;

use crate::{Error, Span, TypeInstance, Value};

impl TypeInstance for Plane {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Plane".into()
    }
}

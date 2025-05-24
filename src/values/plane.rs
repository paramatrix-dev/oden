use anvil::Plane;

use crate::{errors::Error, syntax::Span};

use super::{InnerValue, Value};

impl InnerValue for Plane {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Plane".into()
    }
}

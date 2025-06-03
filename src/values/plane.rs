use anvil::Plane;

use crate::{errors::Error, syntax::Span};

use super::{TypeInstance, Value};

impl TypeInstance for Plane {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Plane".into()
    }
}

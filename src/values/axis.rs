use anvil::Axis3D;

use crate::{errors::Error, syntax::Span};

use super::{InnerValue, Value};

impl InnerValue for Axis3D {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Axis".into()
    }
}

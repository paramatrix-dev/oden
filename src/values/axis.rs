use anvil::Axis;

use crate::{errors::Error, syntax::Span};

use super::{TypeInstance, Value};

impl TypeInstance for Axis<3> {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Axis".into()
    }
}

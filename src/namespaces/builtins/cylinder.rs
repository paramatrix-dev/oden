use std::sync::Arc;

use anvil::Cylinder;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type, check_args},
};

impl Type for Cylinder {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cylinder".into()
    }
}
impl InnerValue for Cylinder {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

fn construct(args: &[Value], span: Span) -> Result<Value, Error> {
    check_args(args, vec!["Length", "Length"], span)?;
    match args {
        [Value::Length(radius), Value::Length(height)] => {
            Ok(Value::Part(Cylinder::from_radius(*radius, *height)))
        }
        _ => unreachable!(),
    }
}

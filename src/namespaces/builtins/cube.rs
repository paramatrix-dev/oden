use std::sync::Arc;

use anvil::Cuboid;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type, check_args},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Cube;
impl Type for Cube {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cube".into()
    }
}
impl InnerValue for Cube {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

fn construct(args: &[Value], span: Span) -> Result<Value, Error> {
    check_args(args, vec!["Length"], span)?;
    match args {
        [Value::Length(size)] => Ok(Value::Part(Cuboid::from_dim(*size, *size, *size))),
        _ => unreachable!(),
    }
}

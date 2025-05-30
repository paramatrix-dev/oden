use std::sync::Arc;

use anvil::Sphere;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type, check_args},
};

impl Type for Sphere {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Sphere".into()
    }
}
impl InnerValue for Sphere {
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
        [Value::Length(radius)] => Ok(Value::Part(Sphere::from_radius(*radius))),
        _ => unreachable!(),
    }
}

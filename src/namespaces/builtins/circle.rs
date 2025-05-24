use std::sync::Arc;

use anvil::Circle;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type, check_args},
};

impl Type for Circle {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Circle".into()
    }
}
impl InnerValue for Circle {
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
        [Value::Length(radius)] => Ok(Value::Sketch(Circle::from_radius(*radius))),
        _ => unreachable!(),
    }
}

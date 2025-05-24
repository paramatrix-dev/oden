use std::sync::Arc;

use anvil::Cuboid;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type, check_args},
};

impl Type for Cuboid {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cuboid".into()
    }
}
impl InnerValue for Cuboid {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

fn construct(args: &[Value], span: Span) -> Result<Value, Error> {
    check_args(args, vec!["Length", "Length", "Length"], span)?;
    match args {
        [Value::Length(x), Value::Length(y), Value::Length(z)] => {
            Ok(Value::Part(Cuboid::from_dim(*x, *y, *z)))
        }
        _ => unreachable!(),
    }
}

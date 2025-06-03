use anvil::Sphere;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{TypeInstance, Type, check_args},
};

impl Type for Sphere {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length"], span)?;
        match args {
            [Value::Length(radius)] => Ok(Value::Part(Sphere::from_radius(*radius))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Sphere".into()
    }
}
impl TypeInstance for Sphere {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

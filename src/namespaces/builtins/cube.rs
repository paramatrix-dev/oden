use anvil::Cuboid;

use crate::{Error, Span, Type, Instance, Value, check_args};

#[derive(Debug, PartialEq, Clone)]
pub struct Cube;
impl Type for Cube {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length"], span)?;
        match args {
            [Value::Length(size)] => Ok(Value::Part(Cuboid::from_dim(*size, *size, *size))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cube".into()
    }
}
impl Instance for Cube {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

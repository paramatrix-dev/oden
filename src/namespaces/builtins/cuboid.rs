use anvil::Cuboid;

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{TypeInstance, Type, check_args},
};

impl Type for Cuboid {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length", "Length", "Length"], span)?;
        match args {
            [Value::Length(x), Value::Length(y), Value::Length(z)] => {
                Ok(Value::Part(Cuboid::from_dim(*x, *y, *z)))
            }
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cuboid".into()
    }
}
impl TypeInstance for Cuboid {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

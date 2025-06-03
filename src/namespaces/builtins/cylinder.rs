use anvil::Cylinder;

use crate::{Error, Span, Type, TypeInstance, Value, check_args};

impl Type for Cylinder {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length", "Length"], span)?;
        match args {
            [Value::Length(radius), Value::Length(height)] => {
                Ok(Value::Part(Cylinder::from_radius(*radius, *height)))
            }
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Cylinder".into()
    }
}
impl TypeInstance for Cylinder {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

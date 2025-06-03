use anvil::Rectangle;

use crate::{Error, Span, Type, Instance, Value, check_args};

impl Type for Rectangle {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length", "Length"], span)?;
        match args {
            [Value::Length(x), Value::Length(y)] => Ok(Value::Sketch(Rectangle::from_dim(*x, *y))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Rectangle".into()
    }
}
impl Instance for Rectangle {
    fn method_call(&self, _: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::FunctionIsNotMethod(span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

use anvil::Plane;

use crate::{Error, Span, Type, TypeInstance, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct PlaneConstructor;

impl Type for PlaneConstructor {
    fn construct(&self, _: &[Value], _: Span) -> Result<Value, Error> {
        unimplemented!()
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Plane".into()
    }
}
impl TypeInstance for PlaneConstructor {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "XY" => Ok(Value::Plane(Plane::xy())),
            "XZ" => Ok(Value::Plane(Plane::xz())),
            "YZ" => Ok(Value::Plane(Plane::yz())),
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        namespaces::PartNamespace,
        syntax::{Expression, tokenize},
    };

    use super::*;

    fn eval_str(input: &str) -> Result<Value, Error> {
        Expression::from_tokens(&tokenize(input, &"".into())?)?.evaluate(&PartNamespace::new())
    }

    #[test]
    fn xy() {
        let input = "Plane.XY()";
        assert_eq!(eval_str(input), Ok(Value::Plane(Plane::xy())))
    }
}

use std::sync::Arc;

use anvil::Axis3D;

use crate::{
    errors::Error,
    syntax::Span,
    values::{InnerValue, Type},
    Value,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AxisConstructor;
impl Type for AxisConstructor {
    fn construct(&self) -> Arc<dyn Fn(&[Value], Span) -> Result<Value, Error> + Send + Sync> {
        Arc::new(construct)
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Axis".into()
    }
}
impl InnerValue for AxisConstructor {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "X" => Ok(Value::Axis(Axis3D::x())),
            "Y" => Ok(Value::Axis(Axis3D::y())),
            "Z" => Ok(Value::Axis(Axis3D::z())),
            _ => Err(Error::UnknownMethod(method.into(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

fn construct(_: &[Value], _: Span) -> Result<Value, Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{
        namespaces::PartNamespace,
        syntax::{tokenize, Expression},
    };

    use super::*;

    fn eval_str(input: &str) -> Result<Value, Error> {
        Expression::from_tokens(&tokenize(input, &"".into())?)?.evaluate(&PartNamespace::new())
    }

    #[test]
    fn xy() {
        let input = "Axis.X()";
        assert_eq!(eval_str(input), Ok(Value::Axis(Axis3D::x())))
    }
}

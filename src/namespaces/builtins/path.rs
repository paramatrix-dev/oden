use anvil::{Path, point};

use crate::{
    Value,
    errors::Error,
    syntax::Span,
    values::{Type, TypeInstance, check_args},
};

#[derive(Clone, Debug, PartialEq)]
pub struct PathConstructor;

impl Type for PathConstructor {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        check_args(args, vec!["Length", "Length"], span)?;
        match args {
            [Value::Length(x), Value::Length(y)] => Ok(Value::Path(Path::at(point!(*x, *y)))),
            _ => unreachable!(),
        }
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
    }
    fn name(&self) -> String {
        "Path".into()
    }
}
impl TypeInstance for PathConstructor {
    fn method_call(&self, method: &str, _: &[Value], span: Span) -> Result<Value, Error> {
        Err(Error::UnknownMethod(method.into(), span))
    }
    fn type_str(&self) -> String {
        "Type".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use crate::{
        namespaces::PartNamespace,
        syntax::{Expression, tokenize},
    };

    use super::*;

    fn eval_str(input: &str) -> Result<Value, Error> {
        Expression::from_tokens(&tokenize(input, &"".into())?)?.evaluate(&PartNamespace::new())
    }

    #[test]
    fn at() {
        let input = "Path(1m, 2m)";
        assert_eq!(
            eval_str(input),
            Ok(Value::Path(Path::at(point!(1.m(), 2.m()))))
        )
    }
}

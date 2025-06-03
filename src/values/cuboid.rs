use anvil::Cuboid;

use crate::{Error, Instance, Span, Type, Value, check_args};

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
impl Instance for Cuboid {
    fn type_str(&self) -> String {
        "Type".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use crate::eval_str;

    use super::*;

    #[test]
    fn call() {
        let actual = eval_str("Cuboid(1m, 2m, 3m)");
        assert_eq!(
            actual,
            Ok(Value::Part(Cuboid::from_dim(1.m(), 2.m(), 3.m())))
        )
    }

    #[test]
    fn unknown_method() {
        let actual = eval_str("Cuboid.UNKNOWN()");
        assert_eq!(
            actual,
            Err(Error::UnknownMethod(
                "UNKNOWN".into(),
                Span(0, 16, "".into())
            ))
        )
    }
}

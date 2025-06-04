use anvil::Cuboid;

use crate::{
    Error, Instance, Span, Type, Value, values::traits::match_args::match_three_length_args,
};

impl Type for Cuboid {
    fn name(&self) -> String {
        "Cuboid".into()
    }
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error> {
        let (x, y, z) = match_three_length_args(args, span)?;
        Ok(Value::Part(Cuboid::from_dim(x, y, z)))
    }
    fn for_namespace(&self) -> (String, crate::Value) {
        (self.name(), Value::Type(Box::new(Self)))
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

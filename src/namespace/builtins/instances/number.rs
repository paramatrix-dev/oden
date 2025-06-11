use std::{ops::Deref, sync::Arc};

use crate::{Callable, Member, from_instance_member, match_args, namespace::traits::Instance};

impl Instance for f64 {
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        let self_clone = *self;
        vec![
            Box::new(Add(Arc::new(self_clone))),
            Box::new(Divide(Arc::new(self_clone))),
            Box::new(Multiply(Arc::new(self_clone))),
            Box::new(Subtract(Arc::new(self_clone))),
        ]
    }
    fn type_name(&self) -> String {
        "Number".into()
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<f64>() {
            Some(o) => self == o,
            None => false,
        }
    }
}
from_instance_member!(f64);

#[derive(Clone, Debug, PartialEq)]
struct Add(Arc<f64>);
impl Callable for Add {
    fn full_name(&self) -> String {
        "Number.add".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() + *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Divide(Arc<f64>);
impl Callable for Divide {
    fn full_name(&self) -> String {
        "Number.divide".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() / *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Multiply(Arc<f64>);
impl Callable for Multiply {
    fn full_name(&self) -> String {
        "Number.multiply".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() * *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Subtract(Arc<f64>);
impl Callable for Subtract {
    fn full_name(&self) -> String {
        "Number.subtract".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() - *other).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;

    #[test]
    fn add() {
        assert_eq!(eval_str("1 + 2"), Ok(3.0.into()))
    }

    #[test]
    fn divide() {
        assert_eq!(eval_str("6 / 2"), Ok(3.0.into()))
    }

    #[test]
    fn multiply() {
        assert_eq!(eval_str("3 * 2"), Ok(6.0.into()))
    }

    #[test]
    fn subtract() {
        assert_eq!(eval_str("3 - 2"), Ok(1.0.into()))
    }
}

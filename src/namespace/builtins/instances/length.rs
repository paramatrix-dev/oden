use std::{ops::Deref, sync::Arc};

use anvil::Length;

use crate::{Callable, Member, from_instance_member, match_args, namespace::traits::Instance};

impl Instance for Length {
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        let self_clone = *self;
        vec![
            Box::new(Add(Arc::new(self_clone))),
            Box::new(Divide(Arc::new(self_clone))),
            Box::new(Multiply(Arc::new(self_clone))),
            Box::new(Subtract(Arc::new(self_clone))),
        ]
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Length>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Length".into()
    }
}
from_instance_member!(Length);

#[derive(Clone, Debug, PartialEq)]
struct Add(Arc<Length>);
impl Callable for Add {
    fn full_name(&self) -> String {
        "Length.add".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Length, args, span);
        Ok((*self.0.deref() + *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Divide(Arc<Length>);
impl Callable for Divide {
    fn full_name(&self) -> String {
        "Length.divide".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() / *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Multiply(Arc<Length>);
impl Callable for Multiply {
    fn full_name(&self) -> String {
        "Length.multiply".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() * *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Subtract(Arc<Length>);
impl Callable for Subtract {
    fn full_name(&self) -> String {
        "Length.subtract".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Length, args, span);
        Ok((*self.0.deref() - *other).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::IntoLength;

    #[test]
    fn add() {
        assert_eq!(eval_str("10m + 5m"), Ok(15.m().into()))
    }

    #[test]
    fn divide() {
        assert_eq!(eval_str("6m / 2"), Ok(3.m().into()))
    }

    #[test]
    fn multiply() {
        assert_eq!(eval_str("3m * 2"), Ok(6.m().into()))
    }

    #[test]
    fn subtract() {
        assert_eq!(eval_str("10m - 3m"), Ok(7.m().into()))
    }
}

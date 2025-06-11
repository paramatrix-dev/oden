use std::{ops::Deref, sync::Arc};

use anvil::Angle;

use crate::{Callable, Member, from_instance_member, match_args, namespace::traits::Instance};

impl Instance for Angle {
    fn type_name(&self) -> String {
        "Angle".into()
    }
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
        match other.downcast_ref::<Angle>() {
            Some(o) => self == o,
            None => false,
        }
    }
}
from_instance_member!(Angle);

#[derive(Clone, Debug, PartialEq)]
struct Add(Arc<Angle>);
impl Callable for Add {
    fn full_name(&self) -> String {
        "Angle.add".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Angle, args, span);
        Ok((*self.0.deref() + *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Divide(Arc<Angle>);
impl Callable for Divide {
    fn full_name(&self) -> String {
        "Angle.divide".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() / *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Multiply(Arc<Angle>);
impl Callable for Multiply {
    fn full_name(&self) -> String {
        "Angle.multiply".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(f64, args, span);
        Ok((*self.0.deref() * *other).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Subtract(Arc<Angle>);
impl Callable for Subtract {
    fn full_name(&self) -> String {
        "Angle.subtract".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Angle, args, span);
        Ok((*self.0.deref() - *other).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::IntoAngle;

    #[test]
    fn add() {
        assert_eq!(eval_str("1rad + 2rad"), Ok(3.rad().into()))
    }

    #[test]
    fn divide() {
        assert_eq!(eval_str("6rad / 2"), Ok(3.rad().into()))
    }

    #[test]
    fn multiply() {
        assert_eq!(eval_str("3rad * 2"), Ok(6.rad().into()))
    }

    #[test]
    fn subtract() {
        assert_eq!(eval_str("3rad - 2rad"), Ok(1.rad().into()))
    }
}

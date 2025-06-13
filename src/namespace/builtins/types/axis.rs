use anvil::Axis;

use crate::{
    from_type_member, match_args,
    namespace::traits::{Callable, Instance, Type},
};

#[derive(Clone, Debug, PartialEq)]
pub struct AxisType;
from_type_member!(AxisType);

impl Type for AxisType {}

impl Callable for AxisType {
    fn full_name(&self) -> String {
        "Axis".into()
    }
}

impl Instance for AxisType {
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        vec![Box::new(X), Box::new(Y), Box::new(Z)]
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct X;
impl Callable for X {
    fn full_name(&self) -> String {
        "Axis.X".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Axis::<3>::x().into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Y;
impl Callable for Y {
    fn full_name(&self) -> String {
        "Axis.Y".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Axis::<3>::y().into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Z;
impl Callable for Z {
    fn full_name(&self) -> String {
        "Axis.Z".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Axis::<3>::z().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::Axis;

    #[test]
    fn x_constructor() {
        assert_eq!(eval_str("Axis.X()"), Ok(Axis::<3>::x().into()))
    }

    #[test]
    fn y_constructor() {
        assert_eq!(eval_str("Axis.Y()"), Ok(Axis::<3>::y().into()))
    }

    #[test]
    fn z_constructor() {
        assert_eq!(eval_str("Axis.Z()"), Ok(Axis::<3>::z().into()))
    }
}

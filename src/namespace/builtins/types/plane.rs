use anvil::Plane;

use crate::{
    from_type_member, match_args,
    namespace::traits::{Callable, Instance, Type},
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlaneType;
impl Type for PlaneType {}
from_type_member!(PlaneType);

impl Callable for PlaneType {
    fn full_name(&self) -> String {
        "Plane".into()
    }
}

impl Instance for PlaneType {
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        vec![Box::new(XY), Box::new(XZ), Box::new(YZ)]
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct XY;
impl Callable for XY {
    fn full_name(&self) -> String {
        "Plane.XY".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Plane::xy().into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct XZ;
impl Callable for XZ {
    fn full_name(&self) -> String {
        "Plane.XZ".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Plane::xz().into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct YZ;
impl Callable for YZ {
    fn full_name(&self) -> String {
        "Plane.YZ".into()
    }
    fn call(
        &self,
        args: &[crate::Member],
        span: crate::Span,
    ) -> Result<crate::Member, crate::Error> {
        match_args!(args, span);
        Ok(Plane::yz().into())
    }
}

#[cfg(test)]
mod tests {
    use anvil::Plane;

    use crate::eval_str;

    #[test]
    fn xy_constructor() {
        assert_eq!(eval_str("Plane.XY()"), Ok(Plane::xy().into()))
    }

    #[test]
    fn xz_constructor() {
        assert_eq!(eval_str("Plane.XZ()"), Ok(Plane::xz().into()))
    }

    #[test]
    fn yz_constructor() {
        assert_eq!(eval_str("Plane.YZ()"), Ok(Plane::yz().into()))
    }
}

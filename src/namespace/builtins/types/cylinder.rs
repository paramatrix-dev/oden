use anvil::{Cylinder, Length};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Cylinder {}
from_type_member!(Cylinder);

impl Callable for Cylinder {
    fn full_name(&self) -> String {
        "Cylinder".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let (radius, height) = match_args!(Length, Length, args, span);
        Ok(Member::Instance(Box::new(Cylinder::from_radius(
            radius, height,
        ))))
    }
}

impl Instance for Cylinder {
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use super::*;
    use crate::eval_str;

    #[test]
    fn construct() {
        assert_eq!(
            eval_str("Cylinder(5m, 6m)"),
            Ok(Cylinder::from_radius(5.m(), 6.m()).into())
        )
    }
}

use anvil::{Length, Sphere};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Sphere {}
from_type_member!(Sphere);

impl Callable for Sphere {
    fn full_name(&self) -> String {
        "Sphere".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let radius = match_args!(Length, args, span);
        Ok(Member::Instance(Box::new(Sphere::from_radius(*radius))))
    }
}

impl Instance for Sphere {
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
            eval_str("Sphere(5m)"),
            Ok(Sphere::from_radius(5.m()).into())
        )
    }
}

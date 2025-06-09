use anvil::{Cylinder, Length};

use crate::{
    Error, Span, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Cylinder {}

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

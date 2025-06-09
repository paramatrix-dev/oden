use anvil::{Length, Sphere};

use crate::{
    Error, Span, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Sphere {}

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

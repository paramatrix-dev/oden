use anvil::{Cuboid, Length};

use crate::{
    Error, Span, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Cuboid {}

impl Callable for Cuboid {
    fn full_name(&self) -> String {
        "Cuboid".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let (x, y, z) = match_args!(Length, Length, Length, args, span);
        Ok(Member::Instance(Box::new(Cuboid::from_dim(x, y, z))))
    }
}

impl Instance for Cuboid {
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

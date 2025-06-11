use anvil::{Cuboid, Length};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Cuboid {}
from_type_member!(Cuboid);

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

#[cfg(test)]
mod tests {
    use anvil::IntoLength;

    use super::*;
    use crate::eval_str;

    #[test]
    fn construct() {
        assert_eq!(
            eval_str("Cuboid(5m, 6m, 7m)"),
            Ok(Cuboid::from_dim(5.m(), 6.m(), 7.m()).into())
        )
    }
}

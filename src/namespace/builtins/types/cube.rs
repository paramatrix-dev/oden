use anvil::{Cube, Length};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Cube {}
from_type_member!(Cube);

impl Callable for Cube {
    fn full_name(&self) -> String {
        "Cube".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let size = match_args!(Length, args, span);
        Ok(Member::Instance(Box::new(Cube::from_size(*size))))
    }
}

impl Instance for Cube {
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
        assert_eq!(eval_str("Cube(5m)"), Ok(Cube::from_size(5.m()).into()))
    }
}

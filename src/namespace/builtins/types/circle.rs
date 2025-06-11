use anvil::{Circle, Length};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Circle {}
from_type_member!(Circle);

impl Callable for Circle {
    fn full_name(&self) -> String {
        "Circle".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let radius = match_args!(Length, args, span);
        Ok(Member::Instance(Box::new(Circle::from_radius(*radius))))
    }
}

impl Instance for Circle {
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
            eval_str("Circle(5m)"),
            Ok(Circle::from_radius(5.m()).into())
        )
    }
}

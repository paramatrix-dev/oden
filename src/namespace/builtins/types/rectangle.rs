use anvil::{Length, Rectangle};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Rectangle {}
from_type_member!(Rectangle);

impl Callable for Rectangle {
    fn full_name(&self) -> String {
        "Rectangle".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let (x, y) = match_args!(Length, Length, args, span);
        Ok(Member::Instance(Box::new(Rectangle::from_dim(x, y))))
    }
}

impl Instance for Rectangle {
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
            eval_str("Rectangle(5m, 6m)"),
            Ok(Rectangle::from_dim(5.m(), 6.m()).into())
        )
    }
}

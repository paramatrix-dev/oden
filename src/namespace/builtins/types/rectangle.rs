use anvil::{Length, Rectangle};

use crate::{
    Error, Span, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

impl Type for Rectangle {}

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

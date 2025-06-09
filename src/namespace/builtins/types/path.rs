use anvil::{Length, Path, point};

use crate::{
    Error, Span, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct PathType;
impl Type for PathType {}

impl Callable for PathType {
    fn full_name(&self) -> String {
        "Path".into()
    }
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        let (x, y) = match_args!(Length, Length, args, span);
        Ok(Member::Instance(Box::new(Path::at(point!(x, y)))))
    }
}

impl Instance for PathType {
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

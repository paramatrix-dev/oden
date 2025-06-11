use anvil::{Length, Path, point};

use crate::{
    Error, Span, from_type_member, match_args,
    namespace::{
        Member,
        traits::{Callable, Instance, Type},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct PathType;
impl Type for PathType {}
from_type_member!(PathType);

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

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::{IntoLength, Path, point};

    #[test]
    fn construct() {
        assert_eq!(
            eval_str("Path(1m, 2m)"),
            Ok(Path::at(point!(1.m(), 2.m())).into())
        )
    }
}

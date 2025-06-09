use crate::namespace::traits::{Callable, Instance, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct PlaneType;
impl Type for PlaneType {}

impl Callable for PlaneType {
    fn full_name(&self) -> String {
        "Plane".into()
    }
}

impl Instance for PlaneType {
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

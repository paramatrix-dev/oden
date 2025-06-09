use crate::namespace::traits::{Callable, Instance, Type};

#[derive(Clone, Debug, PartialEq)]
pub struct AxisType;
impl Type for AxisType {}

impl Callable for AxisType {
    fn full_name(&self) -> String {
        "Axis".into()
    }
}

impl Instance for AxisType {
    fn eq(&self, other: &dyn Instance) -> bool {
        other.downcast_ref::<Self>().is_some()
    }
    fn type_name(&self) -> String {
        "Type".into()
    }
}

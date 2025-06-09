use anvil::Axis;

use crate::namespace::traits::Instance;

impl Instance for Axis<3> {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Axis<3>>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Axis".into()
    }
}

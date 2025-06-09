use anvil::Angle;

use crate::namespace::traits::Instance;

impl Instance for Angle {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Angle>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Angle".into()
    }
}

use anvil::Plane;

use crate::namespace::traits::Instance;

impl Instance for Plane {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Plane>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Plane".into()
    }
}

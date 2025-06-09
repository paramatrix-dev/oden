use anvil::Sketch;

use crate::namespace::traits::Instance;

impl Instance for Sketch {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Sketch>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Sketch".into()
    }
}

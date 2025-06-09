use anvil::Part;

use crate::namespace::traits::Instance;

impl Instance for Part {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Self>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Part".into()
    }
}

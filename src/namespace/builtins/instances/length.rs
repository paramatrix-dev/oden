use anvil::Length;

use crate::namespace::traits::Instance;

impl Instance for Length {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Length>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Length".into()
    }
}

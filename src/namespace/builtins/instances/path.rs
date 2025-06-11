use anvil::Path;

use crate::{from_instance_member, namespace::traits::Instance};

impl Instance for Path {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Path>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Path".into()
    }
}
from_instance_member!(Path);

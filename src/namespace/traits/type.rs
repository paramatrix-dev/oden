use dyn_clone::{DynClone, clone_trait_object};

use crate::namespace::traits::{Callable, Instance};
pub trait Type: Callable + Instance + DynClone {}
clone_trait_object!(Type);
impl PartialEq for Box<dyn Type> {
    fn eq(&self, other: &Self) -> bool {
        self.full_name() == other.full_name()
    }
}

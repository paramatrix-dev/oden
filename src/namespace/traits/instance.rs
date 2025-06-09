use downcast_rs::{Downcast, impl_downcast};
use dyn_clone::{DynClone, clone_trait_object};

use crate::{Error, Span, namespace::traits::Callable};
pub trait Instance: DynClone + Downcast {
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        vec![]
    }
    fn method(&self, name: String, span: &Span) -> Result<Box<dyn Callable>, Error> {
        match self
            .methods()
            .iter()
            .filter(|m| m.short_name() == name)
            .last()
        {
            Some(m) => Ok(m.clone()),
            None => Err(Error::UnknownMethod(name, span.clone())),
        }
    }
    fn type_name(&self) -> String;
    fn eq(&self, other: &dyn Instance) -> bool;
}

clone_trait_object!(Instance);
impl_downcast!(Instance);

impl PartialEq for Box<dyn Instance> {
    fn eq(&self, other: &Self) -> bool {
        Instance::eq(self.as_ref(), other.as_ref())
    }
}

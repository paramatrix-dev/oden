use dyn_clone::DynClone;

use crate::{Instance, errors::Error, syntax::Span, values::Value};
use std::fmt::Debug;

pub trait Type: Instance + Debug + DynClone {
    fn construct(&self, args: &[Value], span: Span) -> Result<Value, Error>;
    fn for_namespace(&self) -> (String, Value);
    fn name(&self) -> String;
}

dyn_clone::clone_trait_object!(Type);

impl PartialEq for dyn Type + '_ {
    fn eq(&self, that: &dyn Type) -> bool {
        self.name() == that.name()
    }
}

impl Instance for Box<dyn Type> {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        (**self).method_call(method, args, span)
    }
    fn type_str(&self) -> String {
        (**self).type_str()
    }
}

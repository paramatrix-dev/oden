use crate::namespace::traits::Instance;

impl Instance for f64 {
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<f64>() {
            Some(o) => self == o,
            None => false,
        }
    }
    fn type_name(&self) -> String {
        "Number".into()
    }
}

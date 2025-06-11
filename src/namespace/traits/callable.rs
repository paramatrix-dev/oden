use downcast_rs::{Downcast, impl_downcast};
use dyn_clone::{DynClone, clone_trait_object};

use crate::{Error, Span, namespace::Member};

pub trait Callable: DynClone + Downcast {
    #[allow(unused_variables)]
    fn call(&self, args: &[Member], span: Span) -> Result<Member, Error> {
        Err(Error::NotCallable(self.full_name(), span))
    }
    fn full_name(&self) -> String;
    fn short_name(&self) -> String {
        match self.full_name().split(".").last() {
            Some(s) => s.into(),
            None => self.full_name(),
        }
    }
}

clone_trait_object!(Callable);
impl_downcast!(Callable);

impl PartialEq for Box<dyn Callable> {
    fn eq(&self, other: &Self) -> bool {
        self.full_name() == other.full_name()
    }
}

#[macro_export]
macro_rules! match_args {
    ($args:expr, $span:expr) => {
        if !$args.is_empty() {
            return Err($crate::Error::Arguments {
                should: vec![],
                is: $args.iter().map(|arg| arg.type_name()).collect(),
                span: $span.clone(),
            });
        }
    };
    ($first:ty, $args:expr, $span:expr) => {
        match $args {
            [$crate::Member::Instance(t1)] => {
                let a1 = match t1.downcast_ref::<$first>() {
                    Some(a) => Ok(a),
                    None => Err($crate::Error::Arguments {
                        should: vec![stringify!($first).into()],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                a1
            }
            _ => {
                return Err($crate::Error::Arguments {
                    should: vec![stringify!($first).into()],
                    is: $args.iter().map(|arg| arg.type_name()).collect(),
                    span: $span.clone(),
                })
            }
        }
    };
    ($first:ty, $second:ty, $args:expr, $span:expr) => {
        match $args {
            [$crate::Member::Instance(t1), $crate::Member::Instance(t2)] => {
                let a1 = match t1.downcast_ref::<$first>() {
                    Some(a) => Ok(*a),
                    None => Err($crate::Error::Arguments {
                        should: vec![stringify!($first).into(), stringify!($second).into()],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                let a2 = match t2.downcast_ref::<$second>() {
                    Some(a) => Ok(*a),
                    None => Err($crate::Error::Arguments {
                        should: vec![stringify!($first).into(), stringify!($second).into()],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                (a1, a2)
            }
            _ => {
                return Err($crate::Error::Arguments {
                    should: vec![stringify!($first).into(), stringify!($second).into()],
                    is: $args.iter().map(|arg| arg.type_name()).collect(),
                    span: $span.clone(),
                })
            }
        }
    };
    ($first:ty, $second:ty, $third:ty, $args:expr, $span:expr) => {
        match $args {
            [
                $crate::Member::Instance(t1),
                $crate::Member::Instance(t2),
                $crate::Member::Instance(t3),
            ] => {
                let a1 = match t1.downcast_ref::<$first>() {
                    Some(a) => Ok(*a),
                    None => Err($crate::Error::Arguments {
                        should: vec![
                            stringify!($first).into(),
                            stringify!($second).into(),
                            stringify!($third).into(),
                        ],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                let a2 = match t2.downcast_ref::<$second>() {
                    Some(a) => Ok(*a),
                    None => Err($crate::Error::Arguments {
                        should: vec![
                            stringify!($first).into(),
                            stringify!($second).into(),
                            stringify!($third).into(),
                        ],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                let a3 = match t3.downcast_ref::<$second>() {
                    Some(a) => Ok(*a),
                    None => Err($crate::Error::Arguments {
                        should: vec![
                            stringify!($first).into(),
                            stringify!($second).into(),
                            stringify!($third).into(),
                        ],
                        is: $args.iter().map(|arg| arg.type_name()).collect(),
                        span: $span.clone(),
                    }),
                }?;
                (a1, a2, a3)
            }
            _ => {
                return Err($crate::Error::Arguments {
                    should: vec![
                        stringify!($first).into(),
                        stringify!($second).into(),
                        stringify!($third).into(),
                    ],
                    is: $args.iter().map(|arg| arg.type_name()).collect(),
                    span: $span.clone(),
                })
            }
        }
    };
}

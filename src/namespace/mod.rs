pub mod builtins;
mod member;
#[allow(clippy::module_inception)]
mod namespace;
pub mod traits;

pub use member::Member;
pub use namespace::PartNamespace;

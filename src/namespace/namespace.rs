use std::{collections::HashMap, fmt::Debug};

use anvil::Part;

use crate::namespace::Member;

/// A hashmap containing all accessible variables and functions.
///
/// This code
/// ```oden
/// size = 5mm
/// ```
/// would add a field called `size` to the PartNamespace and assign it a length Member of 5
/// millimeters. If the variable is then used later in the program, it is fetched from the
/// PartNamespace. Builtin functions like `Cube()` are also inside the PartNamespace and could
/// be overwritten.
#[derive(Clone, PartialEq)]
pub struct PartNamespace(HashMap<String, Member>);
impl Default for PartNamespace {
    /// Return a PartNamespace with the builtin functions and an empty `part` shape included.
    fn default() -> Self {
        PartNamespace([].into())
    }
}
impl PartNamespace {
    /// Return a PartNamespace with the builtin functions and an empty `part` shape included.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns a reference to the Member corresponding to the key.
    pub fn get(&self, k: &String) -> Option<&Member> {
        self.0.get(k)
    }
    /// Inserts a key-Member pair into the map.
    /// If the map did not have this key present, None is returned.
    pub fn insert(&mut self, k: String, v: Member) -> Option<Member> {
        self.0.insert(k, v)
    }
    pub fn part(&self) -> Part {
        match self.get(&"part".into()) {
            Some(Member::Instance(inner)) => match inner.downcast_ref::<Part>() {
                Some(part) => part.clone(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
impl Debug for PartNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    impl PartNamespace {
        pub fn insert_clone(&self, k: String, v: Member) -> Self {
            let mut copy = self.clone();
            copy.insert(k, v);
            copy
        }
    }
}

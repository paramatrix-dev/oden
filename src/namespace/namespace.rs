use std::{collections::HashMap, fmt::Debug};

use anvil::{Circle, Cube, Cuboid, Cylinder, Part, Rectangle, Sphere};

use crate::{AxisType, Callable, PathType, PlaneType, namespace::Member};

/// A hashmap containing all accessible variables and functions.
///
/// This code
/// ```oden
/// size = 5mm
/// ```
/// would add a field called `size` to the Namespace and assign it a length Member of 5
/// millimeters. If the variable is then used later in the program, it is fetched from the
/// Namespace. Builtin functions like `Cube()` are also inside the Namespace and could
/// be overwritten.
#[derive(Clone, Debug, PartialEq)]
pub struct Namespace(HashMap<String, Member>);
impl Default for Namespace {
    /// Return a Namespace with the builtin functions.
    fn default() -> Self {
        Self(
            [
                (AxisType.full_name(), AxisType.into()),
                (Circle.full_name(), Circle.into()),
                (Cube.full_name(), Cube.into()),
                (Cuboid.full_name(), Cuboid.into()),
                (Cylinder.full_name(), Cylinder.into()),
                (PathType.full_name(), PathType.into()),
                (PlaneType.full_name(), PlaneType.into()),
                (Rectangle.full_name(), Rectangle.into()),
                (Sphere.full_name(), Sphere.into()),
            ]
            .into(),
        )
    }
}
impl Namespace {
    /// Return a Namespace with the builtin functions and an empty `part` shape included.
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
    pub fn insert_clone(&self, k: String, v: Member) -> Self {
        let mut copy = self.clone();
        copy.insert(k, v);
        copy
    }
}

/// A namespace with a reserved field for a part.
#[derive(Clone, Debug, PartialEq)]
pub struct PartNamespace(Namespace);
impl Default for PartNamespace {
    /// Return a PartNamespace with the builtin functions and an empty `part` shape included.
    fn default() -> Self {
        Self(Namespace::new().insert_clone("part".into(), Part::empty().into()))
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

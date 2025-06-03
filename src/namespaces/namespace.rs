use std::collections::HashMap;

use anvil::{Circle, Cube, Cuboid, Cylinder, Part, Rectangle, Sphere};

use crate::{AxisType, PathType, Type, Value};

use super::builtins::PlaneConstructor;

/// A hashmap containing all accessible variables and functions.
///
/// This code
/// ```oden
/// size = 5mm
/// ```
/// would add a field called `size` to the PartNamespace and assign it a length value of 5
/// millimeters. If the variable is then used later in the program, it is fetched from the
/// PartNamespace. Builtin functions like `Cube()` are also inside the PartNamespace and could
/// be overwritten.
#[derive(Clone, Debug, PartialEq)]
pub struct PartNamespace(HashMap<String, Value>);
impl Default for PartNamespace {
    /// Return a PartNamespace with the builtin functions and an empty `part` shape included.
    fn default() -> Self {
        PartNamespace(
            [
                ("part".into(), Value::Part(Part::empty())),
                AxisType.for_namespace(),
                Circle.for_namespace(),
                Cube.for_namespace(),
                Cuboid.for_namespace(),
                Cylinder.for_namespace(),
                PathType.for_namespace(),
                PlaneConstructor.for_namespace(),
                Rectangle.for_namespace(),
                Sphere.for_namespace(),
            ]
            .into(),
        )
    }
}
impl PartNamespace {
    /// Return a PartNamespace with the builtin functions and an empty `part` shape included.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, k: &String) -> Option<&Value> {
        self.0.get(k)
    }
    /// Inserts a key-value pair into the map.
    /// If the map did not have this key present, None is returned.
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.0.insert(k, v)
    }
    /// Return the part variable.
    pub fn part(&self) -> Part {
        match self.get(&String::from("part")) {
            Some(Value::Part(part)) => part.clone(),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    impl PartNamespace {
        pub fn insert_clone(&self, k: String, v: Value) -> Self {
            let mut copy = self.clone();
            copy.insert(k, v);
            copy
        }
    }
}

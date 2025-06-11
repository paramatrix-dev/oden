use std::rc::Rc;

use anvil::{Axis, Length, Part, point};

use crate::{Callable, Member, from_instance_member, match_args, namespace::traits::Instance};

impl Instance for Part {
    fn type_name(&self) -> String {
        "Part".into()
    }
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        let rc = Rc::new(self.clone());
        vec![
            Box::new(Add(Rc::clone(&rc))),
            Box::new(CircularPattern(Rc::clone(&rc))),
            Box::new(Intersect(Rc::clone(&rc))),
            Box::new(MoveTo(Rc::clone(&rc))),
            Box::new(Subtract(Rc::clone(&rc))),
        ]
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Self>() {
            Some(o) => self == o,
            None => false,
        }
    }
}
from_instance_member!(Part);

#[derive(Clone, Debug, PartialEq)]
struct Add(Rc<Part>);
impl Callable for Add {
    fn full_name(&self) -> String {
        "Part.add".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Part, args, span);
        Ok((self.0.add(other)).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CircularPattern(Rc<Part>);
impl Callable for CircularPattern {
    fn full_name(&self) -> String {
        "Part.circular_pattern".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let (around, instances) = match_args!(Axis::<3>, f64, args, span);
        Ok((self.0.circular_pattern(around, instances as u8)).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Intersect(Rc<Part>);
impl Callable for Intersect {
    fn full_name(&self) -> String {
        "Part.intersect".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Part, args, span);
        Ok((self.0.intersect(other)).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MoveTo(Rc<Part>);
impl Callable for MoveTo {
    fn full_name(&self) -> String {
        "Part.move_to".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let (x, y, z) = match_args!(Length, Length, Length, args, span);
        Ok((self.0.move_to(point!(x, y, z))).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Subtract(Rc<Part>);
impl Callable for Subtract {
    fn full_name(&self) -> String {
        "Part.subtract".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Part, args, span);
        Ok((self.0.subtract(other)).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::{Axis, Cube, Cuboid, IntoLength, point};

    #[test]
    fn add() {
        assert_eq!(
            eval_str("Cuboid(1m, 1m, 5m).add(Cuboid(1m, 5m, 1m))"),
            Ok(Cuboid::from_dim(1.m(), 1.m(), 5.m())
                .add(&Cuboid::from_dim(1.m(), 5.m(), 1.m()))
                .into())
        )
    }

    #[test]
    fn circular_pattern() {
        assert_eq!(
            eval_str("Cube(1m).move_to(2m, 2m, 2m).circular_pattern(Axis.Z(), 4)"),
            Ok(Cube::from_size(1.m())
                .move_to(point!(2.m(), 2.m(), 2.m()))
                .circular_pattern(Axis::<3>::z(), 4)
                .into())
        )
    }

    #[test]
    fn intersect() {
        assert_eq!(
            eval_str("Cuboid(1m, 1m, 5m).intersect(Cuboid(1m, 5m, 1m))"),
            Ok(Cuboid::from_dim(1.m(), 1.m(), 5.m())
                .intersect(&Cuboid::from_dim(1.m(), 5.m(), 1.m()))
                .into())
        )
    }

    #[test]
    fn move_to() {
        assert_eq!(
            eval_str("Cube(1m).move_to(2m, 3m, 4m)"),
            Ok(Cube::from_size(1.m())
                .move_to(point!(2.m(), 3.m(), 4.m()))
                .into())
        )
    }

    #[test]
    fn subtract() {
        assert_eq!(
            eval_str("Cuboid(1m, 1m, 5m).subtract(Cuboid(1m, 5m, 1m))"),
            Ok(Cuboid::from_dim(1.m(), 1.m(), 5.m())
                .subtract(&Cuboid::from_dim(1.m(), 5.m(), 1.m()))
                .into())
        )
    }
}

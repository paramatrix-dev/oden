use std::rc::Rc;

use anvil::{Length, Plane, Sketch, point};

use crate::{
    Callable, Error, Member, from_instance_member, match_args, namespace::traits::Instance,
};

impl Instance for Sketch {
    fn type_name(&self) -> String {
        "Sketch".into()
    }
    fn methods(&self) -> Vec<Box<dyn Callable>> {
        let rc = Rc::new(self.clone());
        vec![
            Box::new(Add(Rc::clone(&rc))),
            Box::new(Extrude(Rc::clone(&rc))),
            Box::new(Intersect(Rc::clone(&rc))),
            Box::new(MoveTo(Rc::clone(&rc))),
            Box::new(Subtract(Rc::clone(&rc))),
        ]
    }
    fn eq(&self, other: &dyn Instance) -> bool {
        match other.downcast_ref::<Sketch>() {
            Some(o) => self == o,
            None => false,
        }
    }
}
from_instance_member!(Sketch);

#[derive(Clone, Debug, PartialEq)]
struct Add(Rc<Sketch>);
impl Callable for Add {
    fn full_name(&self) -> String {
        "Sketch.add".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Sketch, args, span);
        Ok((self.0.add(other)).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Extrude(Rc<Sketch>);
impl Callable for Extrude {
    fn full_name(&self) -> String {
        "Sketch.extrude".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let (plane, thickness) = match_args!(Plane, Length, args, span);
        match self.0.extrude(plane, thickness) {
            Ok(part) => Ok(part.into()),
            Err(anvil_error) => Error::from_anvil(Err(anvil_error), Some(span)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Intersect(Rc<Sketch>);
impl Callable for Intersect {
    fn full_name(&self) -> String {
        "Sketch.intersect".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Sketch, args, span);
        Ok((self.0.intersect(other)).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MoveTo(Rc<Sketch>);
impl Callable for MoveTo {
    fn full_name(&self) -> String {
        "Sketch.move_to".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let (x, y) = match_args!(Length, Length, args, span);
        Ok((self.0.move_to(point!(x, y))).into())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Subtract(Rc<Sketch>);
impl Callable for Subtract {
    fn full_name(&self) -> String {
        "Sketch.subtract".into()
    }
    fn call(&self, args: &[Member], span: crate::Span) -> Result<Member, crate::Error> {
        let other = match_args!(Sketch, args, span);
        Ok((self.0.subtract(other)).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;
    use anvil::{IntoLength, Plane, Rectangle, point};

    #[test]
    fn add() {
        assert_eq!(
            eval_str("Rectangle(1m, 5m).add(Rectangle(5m, 1m))"),
            Ok(Rectangle::from_dim(1.m(), 5.m())
                .add(&Rectangle::from_dim(5.m(), 1.m()))
                .into())
        )
    }

    #[test]
    fn extrude() {
        assert_eq!(
            eval_str("Rectangle(1m, 2m).extrude(Plane.XY(), 3m)"),
            Ok(Rectangle::from_dim(1.m(), 2.m())
                .extrude(Plane::xy(), 3.m())
                .unwrap()
                .into())
        )
    }

    #[test]
    fn intersect() {
        assert_eq!(
            eval_str("Rectangle(1m, 5m).intersect(Rectangle(5m, 1m))"),
            Ok(Rectangle::from_dim(1.m(), 5.m())
                .intersect(&Rectangle::from_dim(5.m(), 1.m()))
                .into())
        )
    }

    #[test]
    fn move_to() {
        assert_eq!(
            eval_str("Rectangle(1m, 2m).move_to(3m, 4m)"),
            Ok(Rectangle::from_dim(1.m(), 2.m())
                .move_to(point!(3.m(), 4.m()))
                .into())
        )
    }

    #[test]
    fn subtract() {
        assert_eq!(
            eval_str("Rectangle(1m, 5m).subtract(Rectangle(5m, 1m))"),
            Ok(Rectangle::from_dim(1.m(), 5.m())
                .subtract(&Rectangle::from_dim(5.m(), 1.m()))
                .into())
        )
    }
}

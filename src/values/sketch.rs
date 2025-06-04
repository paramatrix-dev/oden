use anvil::{Sketch, point};

use crate::{
    Error, Instance, Span, Value,
    values::traits::match_args::{
        match_angle_arg, match_plane_length_arg, match_sketch_arg, match_two_length_args,
    },
};

impl Instance for Sketch {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => Ok(Value::Sketch(self.add(match_sketch_arg(args, span)?))),
            "extrude" => {
                let (plane, thickness) = match_plane_length_arg(args, span.clone())?;
                Ok(Value::Part(Error::from_anvil(
                    self.extrude(plane, thickness),
                    Some(span),
                )?))
            }
            "intersect" => Ok(Value::Sketch(self.intersect(match_sketch_arg(args, span)?))),
            "move_to" => {
                let (x, y) = match_two_length_args(args, span)?;
                Ok(Value::Sketch(self.move_to(point!(x, y))))
            }
            "rotate" => Ok(Value::Sketch(self.rotate(match_angle_arg(args, span)?))),
            "subtract" => Ok(Value::Sketch(self.subtract(match_sketch_arg(args, span)?))),
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Sketch".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::{Circle, IntoAngle, IntoLength, Square};

    use crate::eval_str;

    use super::*;

    #[test]
    fn instance_method_add() {
        let actual = eval_str("Rectangle(3m, 3m).add(Circle(4m))");
        assert_eq!(
            actual,
            Ok(Value::Sketch(
                Square::from_size(3.m()).add(&Circle::from_radius(4.m()))
            ))
        )
    }

    #[test]
    fn instance_method_intersect() {
        let actual = eval_str("Rectangle(3m, 3m).intersect(Circle(4m))");
        assert_eq!(
            actual,
            Ok(Value::Sketch(
                Square::from_size(3.m()).intersect(&Circle::from_radius(4.m()))
            ))
        )
    }

    #[test]
    fn instance_method_move_to() {
        let actual = eval_str("Circle(1m).move_to(2m, 0m)");
        assert_eq!(
            actual,
            Ok(Value::Sketch(
                Circle::from_radius(1.m()).move_to(point!(2.m(), 0.m()))
            ))
        )
    }

    #[test]
    fn instance_method_rotate() {
        let actual = eval_str("Rectangle(1m, 1m).rotate(45deg)");
        assert_eq!(
            actual,
            Ok(Value::Sketch(Square::from_size(1.m()).rotate(45.deg())))
        )
    }

    #[test]
    fn instance_method_subtract() {
        let actual = eval_str("Rectangle(3m, 3m).subtract(Circle(4m))");
        assert_eq!(
            actual,
            Ok(Value::Sketch(
                Square::from_size(3.m()).subtract(&Circle::from_radius(4.m()))
            ))
        )
    }
}

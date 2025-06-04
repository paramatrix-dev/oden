use anvil::{Part, point};

use crate::{
    Error, Instance, Span, Value,
    values::traits::match_args::{
        match_axis_angle_arg, match_axis_num_arg, match_part_arg, match_three_length_args,
    },
};

impl Instance for Part {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        match method {
            "add" => Ok(Value::Part(self.add(match_part_arg(args, span)?))),
            "circular_pattern" => {
                let (around, instances) = match_axis_num_arg(args, span)?;
                Ok(Value::Part(self.circular_pattern(around, instances as u8)))
            }
            "intersect" => Ok(Value::Part(self.intersect(match_part_arg(args, span)?))),
            "move_to" => {
                let (x, y, z) = match_three_length_args(args, span)?;
                Ok(Value::Part(self.move_to(point!(x, y, z))))
            }
            "rotate_around" => {
                let (around, angle) = match_axis_angle_arg(args, span)?;
                Ok(Value::Part(self.rotate_around(around, angle)))
            }
            "subtract" => Ok(Value::Part(self.subtract(match_part_arg(args, span)?))),
            _ => Err(Error::UnknownMethod(method.to_owned(), span)),
        }
    }
    fn type_str(&self) -> String {
        "Part".into()
    }
}

#[cfg(test)]
mod tests {
    use anvil::{Axis, Cube, Cylinder, IntoAngle, IntoLength};

    use crate::eval_str;

    use super::*;

    #[test]
    fn instance_method_add() {
        let actual = eval_str("Cube(3m).add(Cylinder(1m, 4m))");
        assert_eq!(
            actual,
            Ok(Value::Part(
                Cube::from_size(3.m()).add(&Cylinder::from_radius(1.m(), 4.m()))
            ))
        )
    }

    #[test]
    fn instance_method_circular_pattern() {
        let actual = eval_str("Cube(1m).move_to(2m, 0m, 0m).circular_pattern(Axis.Z(), 4)");
        assert_eq!(
            actual,
            Ok(Value::Part(
                Cube::from_size(1.m())
                    .move_to(point!(2.m(), 0.m(), 0.m()))
                    .circular_pattern(Axis::<3>::z(), 4)
            ))
        )
    }

    #[test]
    fn instance_method_intersect() {
        let actual = eval_str("Cube(3m).intersect(Cylinder(1m, 4m))");
        assert_eq!(
            actual,
            Ok(Value::Part(
                Cube::from_size(3.m()).intersect(&Cylinder::from_radius(1.m(), 4.m()))
            ))
        )
    }

    #[test]
    fn instance_method_move_to() {
        let actual = eval_str("Cube(1m).move_to(2m, 0m, 0m)");
        assert_eq!(
            actual,
            Ok(Value::Part(Cube::from_size(1.m()).move_to(point!(
                2.m(),
                0.m(),
                0.m()
            ))))
        )
    }

    #[test]
    fn instance_method_rotate_around() {
        let actual = eval_str("Cube(1m).move_to(2m, 0m, 0m).rotate_around(Axis.Z(), 90deg)");
        assert_eq!(
            actual,
            Ok(Value::Part(
                Cube::from_size(1.m())
                    .move_to(point!(2.m(), 0.m(), 0.m()))
                    .rotate_around(Axis::<3>::z(), 90.deg())
            ))
        )
    }

    #[test]
    fn instance_method_subtract() {
        let actual = eval_str("Cube(3m).subtract(Cylinder(1m, 4m))");
        assert_eq!(
            actual,
            Ok(Value::Part(
                Cube::from_size(3.m()).subtract(&Cylinder::from_radius(1.m(), 4.m()))
            ))
        )
    }
}

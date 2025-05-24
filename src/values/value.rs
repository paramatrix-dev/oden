use anvil::{Angle, Axis, Length, Part, Path, Plane, Sketch};
use regex::Regex;

use crate::{errors::Error, syntax::Span};

use super::{Type, inner_value::InnerValue};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Angle(Angle),
    Axis(Axis),
    Length(Length),
    Number(f64),
    Part(Part),
    Path(Path),
    Plane(Plane),
    Sketch(Sketch),
    Type(Box<dyn Type>),
}
impl Value {
    pub fn from_str(literal: &str, span: Span) -> Result<Value, Error> {
        let (number, unit) = match split_number_and_unit(literal) {
            Some(v) => v,
            None => todo!(),
        };

        match unit {
            "" => Ok(Value::Number(number)),
            "m" => Ok(Value::Length(Length::from_m(number))),
            "cm" => Ok(Value::Length(Length::from_cm(number))),
            "mm" => Ok(Value::Length(Length::from_mm(number))),
            "deg" => Ok(Value::Angle(Angle::from_deg(number))),
            "rad" => Ok(Value::Angle(Angle::from_rad(number))),
            _ => Err(Error::UnknownUnit(unit.into(), span)),
        }
    }

    pub fn apply(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error> {
        self.inner().method_call(method, args, span)
    }

    pub fn type_str(&self) -> String {
        self.inner().type_str()
    }

    fn inner(&self) -> &dyn InnerValue {
        match self {
            Value::Angle(inner) => inner,
            Value::Axis(inner) => inner,
            Value::Length(inner) => inner,
            Value::Number(inner) => inner,
            Value::Part(inner) => inner,
            Value::Path(inner) => inner,
            Value::Plane(inner) => inner,
            Value::Sketch(inner) => inner,
            Value::Type(inner) => inner,
        }
    }
}

fn split_number_and_unit(literal: &str) -> Option<(f64, &str)> {
    let re = Regex::new(r"^(-?[0-9]*\.?[0-9]+)([a-zA-Z]+)?$").ok()?;
    let caps = re.captures(literal)?;

    let number = caps.get(1)?.as_str().parse::<f64>().ok()?;
    let unit = match caps.get(2) {
        Some(u) => u.as_str(),
        None => "",
    };

    Some((number, unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_number_and_unit_int_mm() {
        assert_eq!(split_number_and_unit("5mm"), Some((5., "mm")))
    }

    #[test]
    fn test_split_number_and_unit_float_mm() {
        assert_eq!(split_number_and_unit("3.15mm"), Some((3.15, "mm")))
    }

    #[test]
    fn test_split_number_and_unit_int_no_unit() {
        assert_eq!(split_number_and_unit("3"), Some((3., "")))
    }

    #[test]
    fn test_split_number_and_unit_float_no_unit() {
        assert_eq!(split_number_and_unit("3.15"), Some((3.15, "")))
    }
}

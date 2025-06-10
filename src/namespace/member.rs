use anvil::{Angle, Length};
use regex::Regex;

use crate::{Error, Span, Type, namespace::traits::Instance};

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Instance(Box<dyn Instance>),
    Type(Box<dyn Type>),
}
impl Member {
    pub fn from_str(literal: &str, span: Span) -> Result<Self, Error> {
        let (number, unit) = match split_number_and_unit(literal) {
            Some(v) => v,
            None => todo!(),
        };

        match unit {
            "" => Ok(Member::Instance(Box::new(number))),
            "m" => Ok(Member::Instance(Box::new(Length::from_m(number)))),
            "cm" => Ok(Member::Instance(Box::new(Length::from_cm(number)))),
            "mm" => Ok(Member::Instance(Box::new(Length::from_mm(number)))),
            "deg" => Ok(Member::Instance(Box::new(Angle::from_deg(number)))),
            "rad" => Ok(Member::Instance(Box::new(Angle::from_rad(number)))),
            _ => Err(Error::UnknownUnit(unit.into(), span)),
        }
    }
    pub fn type_name(&self) -> String {
        match self {
            Member::Instance(inner) => inner.type_name(),
            Member::Type(_) => "Type".into(),
        }
    }
    pub fn method(&self, name: String, args: &[Member], span: &Span) -> Result<Self, Error> {
        match self {
            Self::Instance(inner) => inner.method(name, span)?.call(args, span.clone()),
            Self::Type(inner) => inner.method(name, span)?.call(args, span.clone()),
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

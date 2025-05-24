use anvil::Error as AnvilError;
use std::{error::Error as StdError, fmt, path::PathBuf};

use crate::syntax::Span;

/// The errors that can occurr during compilation.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Occurs when a function or method is called with an incorrect number or arguments.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     part.add(Cube(5mm), Cube(6mm))  // add() only takes a single argument
    /// ```
    ArgumentNumber {
        should: usize,
        is: usize,
        span: Span,
    },

    /// Occurs when an argument to a function or method is of the wrong type.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     part.add(1mm)  // the argument for add() should be a shape not a length
    /// ```
    ArgumentType { should: String, span: Span },

    /// Occurs when a sketch without area is extruded.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     sketch = Rectangle(0mm, 5mm)
    ///     part.add(sketch.extrude(Plane.XY(), 7mm))
    /// ```
    EmptyPart(Span),

    /// Occurs when part of a statement is missing.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     size =
    ///     part.add(Cube(size))
    /// ```
    ExpectedExpression(Span),

    /// Occurs when part of a statement is missing.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     = 10mm
    ///     part.add(Cube(1mm))
    /// ```
    ExpectedIdentifyer(Span),

    /// Occurs when a file could not be opened (probably because the path is incorrect).
    FileNotFound(PathBuf),

    /// Occurs when a function is called as a method.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     Cuboid.add(Cube(5mm))  // Cuboid needs to be called as Cuboid()
    /// ```
    FunctionIsNotMethod(Span),

    /// Occurs when a part could not be written as an STL file.
    StlWrite(PathBuf),

    /// Occurs when a symbol is found, that is not supported by the language.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     size = &5mm  // the ampersand is not a supported symbol
    ///     part.add(Cube(2mm))
    /// ```
    UnexpectedSymbol(Span),

    /// Occurs when an attribute is references that has not been defined.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     size = 5mm
    ///     size.unknown
    /// ```
    UnknownAttribute(String, Span),

    /// Occurs when a function is called that has not been defined.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     Undefined()
    /// ```
    UnknownFunction(String, Span),

    /// Occurs when a method is called that has not been defined.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     part.undefined()
    /// ```
    UnknownMethod(String, Span),

    /// Occurs when a unit is used that is unknown.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     size = 5ly  // light years are not (yet) added as a unit
    ///     part.add(Cube(size))
    /// ```
    UnknownUnit(String, Span),

    /// Occurs when a variable is referenced that has not been defined.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    ///     part.add(Cube(undefined))
    /// ```
    UnknownVariable(String, Span),
}
impl Error {
    pub fn from_anvil<T>(result: Result<T, AnvilError>, span: Option<Span>) -> Result<T, Self> {
        match (result, span) {
            (Ok(val), _) => Ok(val),
            (Err(AnvilError::EmptySketch) | Err(AnvilError::EmptyPart), Some(span)) => {
                Err(Self::EmptyPart(span))
            }
            (Err(AnvilError::StlWrite(path)), _) => Err(Self::StlWrite(path)),
            _ => unimplemented!(),
        }
    }
}

impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Under construction")
    }
}

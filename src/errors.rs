use anvil::Error as AnvilError;
use std::{error::Error as StdError, fmt, path::PathBuf};

use crate::syntax::Span;

/// The errors that can occurr during compilation.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Occurs when the arguments to a callable are not correct.
    ///
    /// # Examples
    ///
    /// For wrong argument type
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "Cube(10deg)";
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(
    ///         Error::Arguments {
    ///             should: vec!["Length".into()],
    ///             is: vec!["Angle".into()],
    ///             span: Span::from((0, 11, input))
    ///         }
    ///     )
    /// )
    /// ```
    ///
    /// For wrong argument number
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "Cube(1m, 2m)";
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(
    ///         Error::Arguments {
    ///             should: vec!["Length".into()],
    ///             is: vec!["Length".into(), "Length".into()],
    ///             span: Span::from((0, 12, input))
    ///         }
    ///     )
    /// )
    Arguments {
        should: Vec<String>,
        is: Vec<String>,
        span: Span,
    },

    /// Occurs when a sketch without area is extruded.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, compile_input, Span};
    ///
    /// let input = "
    ///     part Box:
    ///         sketch = Circle(0m) // circle has zero area
    ///         part.add(sketch.extrude(Plane.XY(), 1m))
    /// ";
    /// assert_eq!(
    ///     compile_input(input),
    ///     Err(Error::EmptyPart(Span::from((84, 114, input))))
    /// )
    /// ```
    EmptyPart(Span),

    /// Occurs when part of a statement is missing.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, compile_input, Span};
    ///
    /// let input = "
    ///     part Box:
    ///         size =
    ///         part.add(Cube(size))
    /// ";
    /// assert_eq!(
    ///     compile_input(input),
    ///     Err(Error::ExpectedExpression(Span::from((23, 29, input))))
    /// )
    /// ```
    ExpectedExpression(Span),

    /// Occurs when part of a statement is missing.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, compile_input, Span};
    ///
    /// let input = "
    ///     part Box:
    ///         = 10mm
    ///         part.add(Cube(size))
    /// ";
    /// assert_eq!(
    ///     compile_input(input),
    ///     Err(Error::ExpectedIdentifyer(Span::from((23, 29, input))))
    /// )
    /// ```
    ExpectedIdentifyer(Span),

    /// Occurs when a file could not be opened (probably because the path is incorrect).
    FileNotFound(PathBuf),

    /// Occurs when a value or type is called that can not be constructed using a call.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "Axis()";
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(Error::NotCallable("Axis".into(), Span::from((0, 6, input))))
    /// )
    /// ```
    NotCallable(String, Span),

    /// Occurs when a part could not be written as an STL file.
    StlWrite(PathBuf),

    /// Occurs when a symbol is found, that is not supported by the language.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, compile_input, Span};
    ///
    /// let input = "
    ///     part Box:
    ///         size = &10mm  // ampersand is not an expected symbol
    ///         part.add(Cube(size))
    /// ";
    /// assert_eq!(
    ///     compile_input(input),
    ///     Err(Error::UnexpectedSymbol(Span::from((30, 31, input))))
    /// )
    /// ```
    UnexpectedSymbol(Span),

    /// Occurs when a function is called that has not been defined.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "do_magic()";
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(Error::UnknownFunction("do_magic".into(), Span::from((0, 10, input))))
    /// )
    /// ```
    UnknownFunction(String, Span),

    /// Occurs when a method is called that has not been defined.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "Axis.undefined()";
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(Error::UnknownMethod("undefined".into(), Span::from((0, 16, input))))
    /// )
    /// ```
    UnknownMethod(String, Span),

    /// Occurs when a unit is used that is unknown.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, eval_str, Span};
    ///
    /// let input = "5ly"; // light years are not (yet) supported
    /// assert_eq!(
    ///     eval_str(input),
    ///     Err(Error::UnknownUnit("ly".into(), Span::from((0, 3, input))))
    /// )
    /// ```
    UnknownUnit(String, Span),

    /// Occurs when a variable is referenced that has not been defined.
    ///
    /// # Example
    /// ```rust
    /// use oden::{Error, compile_input, Span};
    ///
    /// let input = "
    ///     part Box:
    ///         part.add(Cube(size))
    /// ";
    /// assert_eq!(
    ///     compile_input(input),
    ///     Err(Error::UnknownVariable("size".into(), Span::from((37, 41, input))))
    /// )
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

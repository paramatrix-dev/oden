use crate::syntax::{Expression, Span};

/// A basic block of the language that produces a side effect.
///
/// An oden file can be broken down into a vector of statements which are then executed.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement(pub StmtKind, pub Span);

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// Assignment of an expression to a variable.
    ///
    /// # Example
    /// ```oden
    /// size = 5mm
    /// box = Cube(size)
    /// ```
    Assignment(String, Expression),

    /// A statement without content (like a comment).
    ///
    /// # Example
    /// ```oden
    /// // this is a comment and should be ignored
    /// ```
    Empty,

    /// A method call where the return value is set as the new value.
    ///
    /// # Example
    /// ```oden
    /// part.add(Cube(5mm))  // part in the namespace now has an added cube
    /// ```
    Expr(Expression),

    /// The declaration of the part at the beginning of the file.
    ///
    /// # Example
    /// ```oden
    /// part MyPart:
    /// ```
    PartDeclaration(String),
}

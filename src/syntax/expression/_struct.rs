use crate::syntax::Span;

/// A basic building block of the language that can be evaluated to a value and has no side effect.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression(pub ExprKind, pub Span);

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// Reference to a variable namespace (like 'part' or 'length').
    Ident(String),

    /// A function call.
    ///
    /// # Example
    /// ```oden
    /// Cube(5mm)
    /// ```
    Function { name: String, args: Vec<Expression> },

    /// A literal definition (like a length or a number).
    ///
    /// # Example
    /// ```oden
    /// 5mm
    /// ```
    Literal(String),

    /// A method call on an expression.
    ///
    /// # Example
    /// ```oden
    /// part.add(Cube(5mm))
    /// Cube(5mm).move_to(1mm, 0mm, 0mm)
    /// ```
    Method {
        receiver: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
}
impl Expression {
    /// Return the ExprKind of this Expression (the first field).
    ///
    /// # Example
    /// ```rust
    /// use oden::{Expression, ExprKind, Span};
    ///
    /// let expr = Expression(ExprKind::Literal("5mm".into()), Span::from((15, 16)));
    /// assert_eq!(expr.kind(), &ExprKind::Literal("5mm".into()))
    /// ```
    pub fn kind(&self) -> &ExprKind {
        &self.0
    }

    /// Return the ExprKind of this Expression (the first field).
    ///
    /// # Example
    /// ```rust
    /// use oden::{Expression, ExprKind, Span};
    ///
    /// let expr = Expression(ExprKind::Literal("5mm".into()), Span::from((15, 16)));
    /// assert_eq!(expr.span(), &Span::from((15, 16)))
    /// ```
    pub fn span(&self) -> &Span {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Span;

    impl Expression {
        /// Return a literal expression.
        pub fn lit(text: &str) -> Self {
            Expression(ExprKind::Literal(text.into()), Span::empty())
        }
        /// Return an identity expression.
        pub fn ident(text: &str) -> Self {
            Expression(ExprKind::Ident(text.into()), Span::empty())
        }
    }
}

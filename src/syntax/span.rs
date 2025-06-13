use std::sync::Arc;

use super::Token;

/// The range of symbols in a specific file.
///
/// Span is used to trace where in the .oden file a Token / Expression / Statement is defined. This
/// helps in guiding the user to the source of a specific error.
///
/// ```rust
/// use oden::Span;
///
/// let start = 5;  // inclusive
/// let end = 8;    // exclusive
/// let context = "part Box:\n    part.add(Cube(1m))".into();
/// Span::from((start, end, context));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Span(pub usize, pub usize, pub Arc<String>);
impl Span {
    /// Return the start of the Span (the first field).
    ///
    /// ```rust
    /// use oden::Span;
    ///
    /// let span = Span::from((1, 5, "part Box: ..."));
    /// assert_eq!(span.start(), 1)
    /// ```
    pub fn start(&self) -> usize {
        self.0
    }

    /// Return the end of the Span (the second field).
    ///
    /// ```rust
    /// use oden::Span;
    ///
    /// let span = Span::from((1, 5, "part Box: ..."));
    /// assert_eq!(span.end(), 5)
    /// ```
    pub fn end(&self) -> usize {
        self.1
    }

    /// Return the line numbers this span is located in.
    ///
    /// ```rust
    /// use oden::Span;
    ///
    /// let input = "
    /// part Box:
    ///     x = 5m
    ///     y = 6m
    ///     z = 7m
    ///     
    ///     part.add(Cuboid(x, y, z))
    /// ";
    /// assert_eq!(
    ///     Span::from((15, 20, input)).lines(),
    ///     (2, 2)
    /// );
    /// assert_eq!(
    ///     Span::from((37, 78, input)).lines(),
    ///     (4, 6)
    /// );
    /// assert_eq!(
    ///     Span::from((1000, 1005, input)).lines(), // if the lines are out of bounds, the total
    ///     (7, 7)                                   // line number is returned
    /// );
    /// ```
    pub fn lines(&self) -> (usize, usize) {
        let total_lines = self.2.chars().filter(|c| *c == '\n').count();
        let mut start_line = total_lines;
        let mut end_line = total_lines;

        let mut current_line = 0;
        for (i, char) in self.2.chars().enumerate() {
            if char == '\n' {
                current_line += 1;
            }

            if i == self.0 {
                start_line = current_line;
            }
            if i == self.1 {
                end_line = current_line;
            }
        }

        (start_line, end_line)
    }

    /// Return the union of two Spans.
    ///
    /// The new Span has the lowest start value its start and the highest end value as its end. The
    /// path is cloned as it is assumed that only Spans from the same file are merged.
    ///
    /// ```rust
    /// use oden::Span;
    ///
    /// let span1 = Span::from((1, 5, "part Box: ..."));
    /// let span2 = Span::from((4, 8, "part Box: ..."));
    /// assert_eq!(span1.merge(&span2), Span::from((1, 8, "part Box: ...")))
    /// ```
    pub fn merge(&self, other: &Self) -> Self {
        let start: usize = if self.start() < other.start() {
            self.start()
        } else {
            other.start()
        };
        let end: usize = if self.end() > other.end() {
            self.end()
        } else {
            other.end()
        };
        Span(start, end, self.2.clone())
    }
}
impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Span(value.0, value.1, Arc::new("".into()))
    }
}
impl From<(usize, usize, &str)> for Span {
    fn from(value: (usize, usize, &str)) -> Self {
        Span(value.0, value.1, Arc::new(value.2.into()))
    }
}

/// Merge the spans of all tokens together.
pub fn merge_token_span(tokens: &[Token]) -> Span {
    let mut span = tokens
        .first()
        .expect("tokens should not be empty")
        .span()
        .clone();
    tokens
        .iter()
        .for_each(|token| span = span.merge(token.span()));
    span
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Span {
        pub fn empty() -> Self {
            Span(0, 0, Arc::new("".into()))
        }
    }

    #[test]
    fn merge_adjacent() {
        let span = Span::from((5, 10));
        assert_eq!(span.merge(&Span::from((10, 20))), Span::from((5, 20)))
    }

    #[test]
    fn merge_overlapping() {
        let span = Span::from((5, 10));
        assert_eq!(span.merge(&Span::from((7, 20))), Span::from((5, 20)))
    }

    #[test]
    fn merge_contained() {
        let span = Span::from((5, 10));
        assert_eq!(span.merge(&Span::from((7, 10))), Span::from((5, 10)))
    }

    #[test]
    fn merge_outside() {
        let span = Span::from((5, 10));
        assert_eq!(span.merge(&Span::from((20, 30))), Span::from((5, 30)))
    }
}

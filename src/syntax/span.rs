use std::path::PathBuf;

use super::Token;

/// The range of symbols in a specific file.
///
/// Span is used to trace where in the .oden file a Token / Expression / Statement is defined. This
/// helps in guiding the user to the source of a specific error.
///
/// # Example:
/// ```rust
/// use oden::Span;
///
/// let start = 5;  // inclusive
/// let end = 8;    // exclusive
/// let path = "/path/to/file".into();
/// Span(start, end, path);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Span(pub usize, pub usize, pub PathBuf);
impl Span {
    /// Return the start of the Span (the first field).
    ///
    /// # Example
    /// ```rust
    /// use oden::Span;
    ///
    /// let span = Span(1, 5, "/path/to/file".into());
    /// assert_eq!(span.start(), 1)
    /// ```
    pub fn start(&self) -> usize {
        self.0
    }

    /// Return the end of the Span (the second field).
    ///
    /// # Example
    /// ```rust
    /// use oden::Span;
    ///
    /// let span = Span(1, 5, "/path/to/file".into());
    /// assert_eq!(span.end(), 5)
    /// ```
    pub fn end(&self) -> usize {
        self.1
    }

    /// Return the path of the Span (the third field).
    ///
    /// # Example
    /// ```rust
    /// use std::path::PathBuf;
    /// use oden::Span;
    ///
    /// let span = Span(1, 5, "/path/to/file".into());
    /// assert_eq!(span.path(), PathBuf::from("/path/to/file"))
    /// ```
    pub fn path(&self) -> PathBuf {
        self.2.clone()
    }

    /// Return the union of two Spans.
    ///
    /// The new Span has the lowest start value its start and the highest end value as its end. The
    /// path is cloned as it is assumed that only Spans from the same file are merged.
    ///
    /// # Example
    /// ```rust
    /// use oden::Span;
    ///
    /// let span1 = Span(1, 5, "/path/to/file".into());
    /// let span2 = Span(4, 8, "/path/to/file".into());
    /// assert_eq!(span1.merge(&span2), Span(1, 8, "/path/to/file".into()))
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
            Span(0, 0, "".into())
        }
    }

    #[test]
    fn test_adjacent() {
        let span = Span(5, 10, "".into());
        assert_eq!(span.merge(&Span(10, 20, "".into())), Span(5, 20, "".into()))
    }

    #[test]
    fn test_overlapping() {
        let span = Span(5, 10, "".into());
        assert_eq!(span.merge(&Span(7, 20, "".into())), Span(5, 20, "".into()))
    }

    #[test]
    fn test_contained() {
        let span = Span(5, 10, "".into());
        assert_eq!(span.merge(&Span(7, 10, "".into())), Span(5, 10, "".into()))
    }

    #[test]
    fn test_outside() {
        let span = Span(5, 10, "".into());
        assert_eq!(span.merge(&Span(20, 30, "".into())), Span(5, 30, "".into()))
    }
}

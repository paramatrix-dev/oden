use super::Span;
use crate::errors::Error;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

/// The smallest unit of meaning in the oden syntax.
///
/// Tokens can be individual symbols (like parentheses or punctuation), entire references (e.g. to
/// a variable) or literal expressions (e.g. a length). Tokens represent an abstraction layer
/// between the literal text input and wholly formed expressions and statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Token(pub TokenKind, pub Span);
impl Token {
    /// Return the TokenKind of this Token (the first field).
    ///
    /// # Example
    /// ```rust
    /// use oden::{Token, TokenKind, Span};
    ///
    /// let token = Token(TokenKind::Dot, Span::from((15, 16, "part Box: ...")));
    /// assert_eq!(token.kind(), &TokenKind::Dot)
    /// ```
    pub fn kind(&self) -> &TokenKind {
        &self.0
    }

    /// Return the Span of this Token (the second field).
    ///
    /// # Example
    /// ```rust
    /// use oden::{Token, TokenKind, Span};
    ///
    /// let token = Token(TokenKind::Dot, Span::from((15, 16, "part Box: ...")));
    /// assert_eq!(token.span(), &Span::from((15, 16, "part Box: ...")))
    /// ```
    pub fn span(&self) -> &Span {
        &self.1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A '*' symbol.
    Asterisk,
    /// A ';' symbol.
    Colon,
    /// A ',' symbol.
    Comma,
    /// A '.' symbol.
    Dot,
    /// A '//'.
    DoubleSlash,
    /// A '=' symbol.
    Equal,
    /// A reference to a variable, function, or method.
    Ident(String),
    /// A literal expression that can be evaluated by itself (like a length).
    Literal(String),
    /// A linebreak.
    LineBreak,
    /// A '(' symbol
    LParen,
    /// A '-' symbol
    Minus,
    /// A '+' symbol
    Plus,
    /// A ')' symbol
    RParen,
    /// A '/' symbol
    Slash,
}

/// Convert a text input into tokens.
#[allow(clippy::ptr_arg)]
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let chars: Vec<char> = input.chars().collect();
    let context: Arc<String> = Arc::new(input.into());
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < chars.len() {
        let (token, advance) = parse_next_token(&chars, pos, context.clone())?;
        if let Some(t) = token {
            tokens.push(t)
        }
        pos += advance;
    }

    Ok(tokens)
}

fn parse_next_token(
    chars: &[char],
    pos: usize,
    context: Arc<String>,
) -> Result<(Option<Token>, usize), Error> {
    let start = pos;
    let ch = chars[pos];

    match ch {
        '.' => Ok((
            Some(Token(TokenKind::Dot, Span(pos, pos + 1, context.clone()))),
            1,
        )),
        ',' => Ok((
            Some(Token(TokenKind::Comma, Span(pos, pos + 1, context.clone()))),
            1,
        )),
        '(' => Ok((
            Some(Token(
                TokenKind::LParen,
                Span(pos, pos + 1, context.clone()),
            )),
            1,
        )),
        ')' => Ok((
            Some(Token(
                TokenKind::RParen,
                Span(pos, pos + 1, context.clone()),
            )),
            1,
        )),
        '=' => Ok((
            Some(Token(TokenKind::Equal, Span(pos, pos + 1, context.clone()))),
            1,
        )),
        ':' => Ok((
            Some(Token(TokenKind::Colon, Span(pos, pos + 1, context.clone()))),
            1,
        )),
        '+' => Ok((
            Some(Token(TokenKind::Plus, Span(pos, pos + 1, context.clone()))),
            1,
        )),
        '-' => {
            let next_char = chars.get(pos + 1);
            if next_char.is_some_and(|c| c.is_ascii_digit()) {
                let (literal, end) = parse_literal(chars, pos);
                Ok((
                    Some(Token(
                        TokenKind::Literal(literal),
                        Span(start, end, context.clone()),
                    )),
                    end - start,
                ))
            } else {
                Ok((
                    Some(Token(TokenKind::Minus, Span(pos, pos + 1, context.clone()))),
                    1,
                ))
            }
        }
        '*' => Ok((
            Some(Token(
                TokenKind::Asterisk,
                Span(pos, pos + 1, context.clone()),
            )),
            1,
        )),
        '\n' => Ok((
            Some(Token(
                TokenKind::LineBreak,
                Span(pos, pos + 1, context.clone()),
            )),
            1,
        )),
        '/' => {
            let next_char = chars.get(pos + 1);
            match next_char {
                Some('/') => Ok((
                    Some(Token(
                        TokenKind::DoubleSlash,
                        Span(pos, pos + 2, context.clone()),
                    )),
                    2,
                )),
                _ => Ok((
                    Some(Token(TokenKind::Slash, Span(pos, pos + 1, context.clone()))),
                    1,
                )),
            }
        }
        ch if ch.is_whitespace() => Ok((None, 1)),
        ch if ch.is_ascii_alphabetic() => {
            let mut end = pos + 1;
            while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
                end += 1;
            }

            let identifyer = chars_to_string(chars[start..end].to_vec());
            let advance = end - start;
            Ok((
                Some(Token(
                    TokenKind::Ident(identifyer),
                    Span(start, end, context.clone()),
                )),
                advance,
            ))
        }
        ch if ch.is_ascii_digit() => {
            let (literal, end) = parse_literal(chars, pos);
            Ok((
                Some(Token(
                    TokenKind::Literal(literal),
                    Span(start, end, context.clone()),
                )),
                end - start,
            ))
        }
        _ => Err(Error::UnexpectedSymbol(Span(pos, pos + 1, context.clone()))),
    }
}

fn parse_literal(chars: &[char], pos: usize) -> (String, usize) {
    let mut end = pos + 1;
    while literal_continues(chars, &end) {
        end += 1;
    }

    let literal = chars_to_string(chars[pos..end].to_vec());
    (literal.to_string(), end)
}

fn literal_continues(chars: &[char], pos: &usize) -> bool {
    if pos >= &chars.len() {
        return false;
    }
    let next_char = chars[*pos];
    if next_char.is_ascii_alphanumeric() {
        return true;
    }
    if next_char == '.' {
        return true;
    }
    false
}

fn chars_to_string(chars: Vec<char>) -> String {
    let mut string = String::new();
    chars.iter().for_each(|char| string.push(char.to_owned()));
    string
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Asterisk => write!(f, "Asterisk"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::DoubleSlash => write!(f, "DoubleSlash"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::Ident(val) => write!(f, "Ident({})", val),
            TokenKind::Literal(val) => write!(f, "Literal({})", val),
            TokenKind::LineBreak => write!(f, "LineBreak"),
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::Slash => write!(f, "Slash"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_identifyer() {
        let input = "height";
        assert_eq!(
            tokenize(input),
            Ok(vec![Token(
                TokenKind::Ident("height".into()),
                Span::from((0, 6, input))
            )])
        )
    }

    #[test]
    fn test_length_literal() {
        let input = "5mm";
        assert_eq!(
            tokenize(input),
            Ok(vec![Token(
                TokenKind::Literal("5mm".into()),
                Span::from((0, 3, input))
            )])
        )
    }

    #[test]
    fn test_length_literal_with_period() {
        let input = "73.1234mm";
        assert_eq!(
            tokenize(input),
            Ok(vec![Token(
                TokenKind::Literal("73.1234mm".into()),
                Span::from((0, 9, input))
            )])
        )
    }

    #[test]
    fn test_two_identifyers_separated_by_dot() {
        let input = "part.add";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Ident("part".into()), Span::from((0, 4, input))),
                Token(TokenKind::Dot, Span::from((4, 5, input))),
                Token(TokenKind::Ident("add".into()), Span::from((5, 8, input))),
            ])
        )
    }

    #[test]
    fn test_assignment() {
        let input = "height = 5mm";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Ident("height".into()), Span::from((0, 6, input))),
                Token(TokenKind::Equal, Span::from((7, 8, input))),
                Token(TokenKind::Literal("5mm".into()), Span::from((9, 12, input))),
            ])
        )
    }

    #[test]
    fn test_addition() {
        let input = "1 + 2";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Literal("1".into()), Span::from((0, 1, input))),
                Token(TokenKind::Plus, Span::from((2, 3, input))),
                Token(TokenKind::Literal("2".into()), Span::from((4, 5, input))),
            ])
        )
    }

    #[test]
    fn test_subtraction() {
        let input = "1 - 2";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Literal("1".into()), Span::from((0, 1, input))),
                Token(TokenKind::Minus, Span::from((2, 3, input))),
                Token(TokenKind::Literal("2".into()), Span::from((4, 5, input))),
            ])
        )
    }

    #[test]
    fn test_multiplication() {
        let input = "1 * 2";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Literal("1".into()), Span::from((0, 1, input))),
                Token(TokenKind::Asterisk, Span::from((2, 3, input))),
                Token(TokenKind::Literal("2".into()), Span::from((4, 5, input))),
            ])
        )
    }

    #[test]
    fn test_division() {
        let input = "1 / 2";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Literal("1".into()), Span::from((0, 1, input))),
                Token(TokenKind::Slash, Span::from((2, 3, input))),
                Token(TokenKind::Literal("2".into()), Span::from((4, 5, input))),
            ])
        )
    }

    #[test]
    fn test_comment() {
        let input = "// a_comment";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::DoubleSlash, Span::from((0, 2, input))),
                Token(
                    TokenKind::Ident("a_comment".into()),
                    Span::from((3, 12, input))
                ),
            ])
        )
    }

    #[test]
    fn test_part_declaration() {
        let input = "part Box:";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Ident("part".into()), Span::from((0, 4, input))),
                Token(TokenKind::Ident("Box".into()), Span::from((5, 8, input))),
                Token(TokenKind::Colon, Span::from((8, 9, input))),
            ])
        )
    }

    #[test]
    fn test_function_call_without_args() {
        let input = "construct()";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(
                    TokenKind::Ident("construct".into()),
                    Span::from((0, 9, input))
                ),
                Token(TokenKind::LParen, Span::from((9, 10, input))),
                Token(TokenKind::RParen, Span::from((10, 11, input))),
            ])
        )
    }

    #[test]
    fn test_function_call_with_args() {
        let input = "Rectangle(5mm, 6mm)";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(
                    TokenKind::Ident("Rectangle".into()),
                    Span::from((0, 9, input))
                ),
                Token(TokenKind::LParen, Span::from((9, 10, input))),
                Token(
                    TokenKind::Literal("5mm".into()),
                    Span::from((10, 13, input))
                ),
                Token(TokenKind::Comma, Span::from((13, 14, input))),
                Token(
                    TokenKind::Literal("6mm".into()),
                    Span::from((15, 18, input))
                ),
                Token(TokenKind::RParen, Span::from((18, 19, input))),
            ])
        )
    }

    #[test]
    fn test_multiline_function_call() {
        let input = "
        Rectangle(
            5mm,
            6mm
        )";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::LineBreak, Span::from((0, 1, input))),
                Token(
                    TokenKind::Ident("Rectangle".into()),
                    Span::from((9, 18, input))
                ),
                Token(TokenKind::LParen, Span::from((18, 19, input))),
                Token(TokenKind::LineBreak, Span::from((19, 20, input))),
                Token(
                    TokenKind::Literal("5mm".into()),
                    Span::from((32, 35, input))
                ),
                Token(TokenKind::Comma, Span::from((35, 36, input))),
                Token(TokenKind::LineBreak, Span::from((36, 37, input))),
                Token(
                    TokenKind::Literal("6mm".into()),
                    Span::from((49, 52, input))
                ),
                Token(TokenKind::LineBreak, Span::from((52, 53, input))),
                Token(TokenKind::RParen, Span::from((61, 62, input))),
            ])
        )
    }

    #[test]
    fn test_two_literals_separated_with_linebreak() {
        let input = "5mm
        6mm";
        assert_eq!(
            tokenize(input),
            Ok(vec![
                Token(TokenKind::Literal("5mm".into()), Span::from((0, 3, input))),
                Token(TokenKind::LineBreak, Span::from((3, 4, input))),
                Token(
                    TokenKind::Literal("6mm".into()),
                    Span::from((12, 15, input))
                ),
            ])
        )
    }

    #[test]
    fn test_unexpected_symbol() {
        let input = "&";
        assert_eq!(
            tokenize(input),
            Err(Error::UnexpectedSymbol(Span::from((0, 1, input))))
        )
    }
}

use crate::{
    errors::Error,
    syntax::{Expression, Span, Token, TokenKind, span::merge_token_span},
};

use super::_struct::{Statement, StmtKind};

impl Statement {
    /// Construct a statement from a vector of tokens.
    pub fn from_tokens(tokens: &Vec<Token>) -> Result<Self, Error> {
        let span = merge_token_span(tokens);
        let tokens = filter_out_comment(tokens);
        if tokens.is_empty() {
            return Ok(Statement(StmtKind::Empty, span));
        }

        match extract_token_kinds(&tokens).as_slice() {
            [
                TokenKind::Ident(_),
                TokenKind::Ident(name),
                TokenKind::Colon,
            ] => Ok(Statement(StmtKind::PartDeclaration(name.clone()), span)),
            [TokenKind::Ident(name), TokenKind::Equal, ..] => {
                assignment_from_tokens(name, &tokens, span)
            }
            [TokenKind::Ident(_), TokenKind::Colon] => Err(Error::ExpectedIdentifyer(span)),
            [TokenKind::Equal, ..] => Err(Error::ExpectedIdentifyer(span)),
            _ => Ok(Statement(
                StmtKind::Expr(Expression::from_tokens(&tokens)?),
                span,
            )),
        }
    }
}

fn filter_out_comment(tokens: &Vec<Token>) -> Vec<Token> {
    let mut token_buffer = vec![];
    let mut is_comment = false;
    for token in tokens {
        match token.kind() {
            TokenKind::DoubleSlash => is_comment = true,
            TokenKind::LineBreak => is_comment = false,
            _ => (),
        }
        if !is_comment {
            token_buffer.push(token.clone());
        }
    }
    token_buffer
}

fn extract_token_kinds(tokens: &[Token]) -> Vec<TokenKind> {
    tokens.iter().map(|t| t.kind().clone()).collect()
}

fn assignment_from_tokens(name: &str, tokens: &[Token], span: Span) -> Result<Statement, Error> {
    if tokens.len() > 2 {
        Ok(Statement(
            StmtKind::Assignment(
                name.to_owned(),
                Expression::from_tokens(&tokens[2..].to_vec())?,
            ),
            span,
        ))
    } else {
        Err(Error::ExpectedExpression(span))
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{Span, expression::ExprKind};

    use super::*;

    fn token(t: TokenKind) -> Token {
        Token(t, Span::empty())
    }

    #[test]
    fn part_declaration() {
        let tokens = vec![
            token(TokenKind::Ident("part".into())),
            token(TokenKind::Ident("Box".into())),
            token(TokenKind::Colon),
        ];
        assert_eq!(
            Statement::from_tokens(&tokens),
            Ok(Statement(
                StmtKind::PartDeclaration("Box".into()),
                Span::empty()
            ))
        )
    }

    #[test]
    fn assignment() {
        let tokens = vec![
            token(TokenKind::Ident("height".into())),
            token(TokenKind::Equal),
            token(TokenKind::Literal("5mm".into())),
        ];
        assert_eq!(
            Statement::from_tokens(&tokens),
            Ok(Statement(
                StmtKind::Assignment(
                    "height".into(),
                    Expression(ExprKind::Literal("5mm".into()), Span::empty())
                ),
                Span::empty()
            ))
        )
    }

    #[test]
    fn method_from_expression_two_iterative() {
        let tokens = vec![
            token(TokenKind::Ident("Rectangle".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::Comma),
            token(TokenKind::Literal("6mm".into())),
            token(TokenKind::RParen),
            token(TokenKind::Dot),
            token(TokenKind::Ident("add".into())),
            token(TokenKind::LParen),
            token(TokenKind::Ident("cube".into())),
            token(TokenKind::RParen),
            token(TokenKind::Dot),
            token(TokenKind::Ident("add".into())),
            token(TokenKind::LParen),
            token(TokenKind::Ident("cylinder".into())),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Statement::from_tokens(&tokens),
            Ok(Statement(
                StmtKind::Expr(Expression::from_tokens(&tokens).unwrap()),
                Span::empty()
            ))
        )
    }
}

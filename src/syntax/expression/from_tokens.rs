use super::{ExprKind, Expression};
use crate::{
    errors::Error,
    syntax::{Span, Token, TokenKind, span::merge_token_span},
};

impl Expression {
    /// Construct an Expression from a vector of tokens.
    pub fn from_tokens(tokens: &Vec<Token>) -> Result<Self, Error> {
        let span = merge_token_span(tokens);
        match extract_token_kinds(tokens).as_slice() {
            _ if tokens_are_method_call(tokens) => method_from_tokens(tokens, span),
            [TokenKind::Literal(value)] => Ok(Expression(ExprKind::Literal(value.clone()), span)),
            [TokenKind::Ident(name)] => Ok(Expression(ExprKind::Ident(name.clone()), span)),
            [TokenKind::Ident(name), TokenKind::LParen, ..] => function_from_tokens(name, tokens),
            _ if tokens_are_math_expression(tokens) => math_from_tokens(tokens, span),
            _ => todo!(),
        }
    }
}

fn extract_token_kinds(tokens: &[Token]) -> Vec<TokenKind> {
    tokens.iter().map(|t| t.kind().clone()).collect()
}

fn tokens_are_method_call(tokens: &Vec<Token>) -> bool {
    let mut bracket_depth = 0;
    for token in tokens {
        let token_kind = token.kind().clone();
        match (bracket_depth, token_kind) {
            (_, TokenKind::LParen) => bracket_depth += 1,
            (_, TokenKind::RParen) => bracket_depth -= 1,
            (0, TokenKind::Dot) => return true,
            _ => (),
        }
    }
    false
}

fn tokens_are_math_expression(tokens: &[Token]) -> bool {
    let token_kinds = extract_token_kinds(tokens);
    token_kinds.contains(&TokenKind::Plus)
        || token_kinds.contains(&TokenKind::Minus)
        || token_kinds.contains(&TokenKind::Asterisk)
        || token_kinds.contains(&TokenKind::Slash)
}

fn method_from_tokens(tokens: &[Token], span: Span) -> Result<Expression, Error> {
    let (receiver_tokens, call_tokens) = split_at_last_top_level_dot(tokens);
    let token_kinds = extract_token_kinds(&call_tokens);
    let (method, call_tokens) = match token_kinds.as_slice() {
        [TokenKind::Ident(method), ..] => (method, call_tokens[1..].to_vec()),
        _ => {
            return Err(Error::ExpectedExpression(span));
        }
    };
    Ok(Expression(
        ExprKind::Method {
            receiver: Box::new(Expression::from_tokens(&receiver_tokens)?),
            method: method.to_owned(),
            args: call_args_from_tokens(&call_tokens)?,
        },
        span,
    ))
}

fn function_from_tokens(name: &String, tokens: &[Token]) -> Result<Expression, Error> {
    let call_tokens = tokens[1..].to_vec();
    Ok(Expression(
        ExprKind::Function {
            name: name.to_owned(),
            args: call_args_from_tokens(&call_tokens)?,
        },
        merge_token_span(tokens),
    ))
}

fn math_from_tokens(tokens: &[Token], span: Span) -> Result<Expression, Error> {
    let mut left_side_tokens = vec![];
    let mut right_side_tokens = vec![];
    let mut method = String::new();
    let mut bracket_level = 0;

    let tokens = remove_encompassing_parenthesis(tokens);
    for (i, token) in tokens.iter().enumerate() {
        match (token.kind(), bracket_level) {
            (TokenKind::LParen, _) => bracket_level += 1,
            (TokenKind::RParen, _) => bracket_level -= 1,
            (TokenKind::Plus, 0) => {
                method = "add".into();
                left_side_tokens = tokens[..i].into();
                right_side_tokens = tokens[i + 1..].into();
            }
            (TokenKind::Minus, 0) => {
                method = "subtract".into();
                left_side_tokens = tokens[..i].into();
                right_side_tokens = tokens[i + 1..].into();
            }
            (TokenKind::Asterisk, 0) => {
                if &method == "add" || &method == "subtract" {
                    continue;
                }
                method = "multiply".into();
                left_side_tokens = tokens[..i].into();
                right_side_tokens = tokens[i + 1..].into();
            }
            (TokenKind::Slash, 0) => {
                if &method == "add" || &method == "subtract" {
                    continue;
                }
                method = "divide".into();
                left_side_tokens = tokens[..i].into();
                right_side_tokens = tokens[i + 1..].into();
            }
            _ => (),
        }
    }
    Ok(Expression(
        ExprKind::Method {
            receiver: Box::new(Expression::from_tokens(&left_side_tokens)?),
            method,
            args: vec![Expression::from_tokens(&right_side_tokens)?],
        },
        span,
    ))
}

fn remove_encompassing_parenthesis(tokens: &[Token]) -> &[Token] {
    let token_kinds = extract_token_kinds(tokens);
    if (token_kinds.first(), token_kinds.last())
        == (Some(&TokenKind::LParen), Some(&TokenKind::RParen))
    {
        &tokens[1..tokens.len() - 1]
    } else {
        tokens
    }
}

fn call_args_from_tokens(call_tokens: &Vec<Token>) -> Result<Vec<Expression>, Error> {
    let mut args = vec![];
    let mut arg_tokens: Vec<Token> = vec![];
    let mut bracket_depth = 0;
    for token in call_tokens {
        let token_kind = token.kind().clone();
        match (bracket_depth, token_kind) {
            (0, TokenKind::LParen) => bracket_depth += 1,
            (0, _) => (),
            (1, TokenKind::Comma) => {
                if !arg_tokens.is_empty() {
                    args.push(Expression::from_tokens(&arg_tokens)?);
                }
                arg_tokens = vec![];
            }
            (1, TokenKind::RParen) => {
                if !arg_tokens.is_empty() {
                    args.push(Expression::from_tokens(&arg_tokens)?);
                }
                arg_tokens = vec![];
                bracket_depth -= 1;
            }
            (_, TokenKind::LParen) => {
                bracket_depth += 1;
                arg_tokens.push(token.clone());
            }
            (_, TokenKind::RParen) => {
                bracket_depth -= 1;
                arg_tokens.push(token.clone());
            }
            _ => arg_tokens.push(token.clone()),
        }
    }
    Ok(args)
}

fn split_at_last_top_level_dot(tokens: &[Token]) -> (Vec<Token>, Vec<Token>) {
    let mut tokens_after_last_top_level_dot = vec![];
    let mut bracket_depth = 0;
    for token in tokens.iter().rev() {
        match (bracket_depth, token.kind()) {
            (_, TokenKind::LParen) => {
                bracket_depth += 1;
            }
            (_, TokenKind::RParen) => {
                bracket_depth -= 1;
            }
            (0, TokenKind::Dot) => {
                break;
            }
            _ => (),
        }
        tokens_after_last_top_level_dot.push(token.clone());
    }
    tokens_after_last_top_level_dot.reverse();
    let remainder = tokens
        .iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| i > &tokens_after_last_top_level_dot.len())
        .map(|(_, t)| t.clone())
        .rev()
        .collect();

    (remainder, tokens_after_last_top_level_dot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Span;

    fn token(t: TokenKind) -> Token {
        Token(t, Span::empty())
    }

    #[test]
    fn numeric() {
        let tokens = vec![token(TokenKind::Literal("3.14".into()))];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(ExprKind::Literal("3.14".into()), Span::empty()))
        )
    }

    #[test]
    fn length() {
        let tokens = vec![token(TokenKind::Literal("5mm".into()))];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(ExprKind::Literal("5mm".into()), Span::empty()))
        )
    }

    #[test]
    fn angle() {
        let tokens = vec![token(TokenKind::Literal("90deg".into()))];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(ExprKind::Literal("90deg".into()), Span::empty()))
        )
    }

    #[test]
    fn identifyer() {
        let tokens = vec![token(TokenKind::Ident("height".into()))];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(ExprKind::Ident("height".into()), Span::empty()))
        )
    }

    #[test]
    fn empty_call() {
        let tokens = vec![
            token(TokenKind::Ident("construct".into())),
            token(TokenKind::LParen),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Function {
                    name: "construct".into(),
                    args: [].into()
                },
                Span::empty()
            ))
        )
    }

    #[test]
    fn call_single_arg() {
        let tokens = vec![
            token(TokenKind::Ident("Sphere".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Function {
                    name: "Sphere".into(),
                    args: [Expression::lit("5mm")].into()
                },
                Span::empty()
            ))
        )
    }

    #[test]
    fn call_two_args() {
        let tokens = vec![
            token(TokenKind::Ident("Rectangle".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::Comma),
            token(TokenKind::Literal("6mm".into())),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Function {
                    name: "Rectangle".into(),
                    args: [Expression::lit("5mm"), Expression::lit("6mm")].into()
                },
                Span::empty()
            ))
        )
    }

    #[test]
    fn nested_args() {
        let tokens = vec![
            token(TokenKind::Ident("add".into())),
            token(TokenKind::LParen),
            token(TokenKind::Ident("Rectangle".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::Comma),
            token(TokenKind::Literal("6mm".into())),
            token(TokenKind::RParen),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Function {
                    name: "add".into(),
                    args: [Expression(
                        ExprKind::Function {
                            name: "Rectangle".into(),
                            args: [Expression::lit("5mm"), Expression::lit("6mm")].into()
                        },
                        Span::empty()
                    )]
                    .into()
                },
                Span::empty()
            ))
        )
    }

    #[test]
    fn method_single_arg() {
        let tokens = vec![
            token(TokenKind::Ident("part".into())),
            token(TokenKind::Dot),
            token(TokenKind::Ident("add".into())),
            token(TokenKind::LParen),
            token(TokenKind::Ident("cube".into())),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Method {
                    receiver: Box::new(Expression::ident("part")),
                    method: "add".into(),
                    args: vec![Expression::ident("cube")]
                },
                Span::empty()
            ))
        )
    }

    #[test]
    fn method_from_expression() {
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
        ];
        assert_eq!(
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Method {
                    receiver: Box::new(Expression(
                        ExprKind::Function {
                            name: "Rectangle".into(),
                            args: [Expression::lit("5mm"), Expression::lit("6mm")].into()
                        },
                        Span::empty()
                    )),
                    method: "add".into(),
                    args: vec![Expression::ident("cube")]
                },
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
            Expression::from_tokens(&tokens),
            Ok(Expression(
                ExprKind::Method {
                    receiver: Box::new(Expression(
                        ExprKind::Method {
                            receiver: Box::new(Expression(
                                ExprKind::Function {
                                    name: "Rectangle".into(),
                                    args: [Expression::lit("5mm"), Expression::lit("6mm")].into()
                                },
                                Span::empty()
                            )),
                            method: "add".into(),
                            args: vec![Expression::ident("cube")]
                        },
                        Span::empty()
                    )),
                    method: "add".into(),
                    args: vec![Expression::ident("cylinder")]
                },
                Span::empty()
            ))
        )
    }
}

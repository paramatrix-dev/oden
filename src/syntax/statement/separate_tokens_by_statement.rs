use crate::syntax::{Token, TokenKind};

/// Split up the tokens of an oden file by statement.
///
/// The resulting subvectors can then be used in `Statement::from_tokens()`.
pub fn separate_tokens_by_statement(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut separated_tokens = vec![];
    let mut buffer = vec![];
    for (i, token) in tokens.iter().enumerate() {
        buffer.push(token.clone());

        if !statement_continues(&tokens, i) {
            separated_tokens.push(buffer);
            buffer = vec![];
        }
    }

    filter_out_linebreaks(separated_tokens)
}

fn statement_continues(tokens: &[Token], pos: usize) -> bool {
    let token = match tokens.get(pos) {
        Some(t) => t.kind().clone(),
        None => return false,
    };
    let next_token = match tokens.get(pos + 1) {
        Some(t) => t.kind().clone(),
        None => return false,
    };
    let mut bracket_level = 0;
    for token in tokens[..pos].iter().map(|t| t.kind().clone()) {
        match token {
            TokenKind::LParen => bracket_level += 1,
            TokenKind::RParen => bracket_level -= 1,
            _ => (),
        }
    }

    if next_token == TokenKind::Dot {
        return true;
    }
    if bracket_level == 0 && token == TokenKind::LineBreak {
        return false;
    }
    true
}

fn filter_out_linebreaks(v: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
    v.into_iter()
        .map(|subv| {
            subv.into_iter()
                .filter(|token| token.kind() != &TokenKind::LineBreak)
                .collect::<Vec<Token>>()
        })
        .filter(|subv| !subv.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Span;

    fn token(t: TokenKind) -> Token {
        Token(t, Span::empty())
    }

    #[test]
    fn single_statement() {
        let tokens = vec![
            token(TokenKind::Ident("part".into())),
            token(TokenKind::Ident("Box".into())),
            token(TokenKind::Colon),
        ];
        assert_eq!(separate_tokens_by_statement(tokens.clone()), vec![tokens])
    }

    #[test]
    fn assignment_after_part_decl() {
        let tokens = vec![
            token(TokenKind::Ident("part".into())),
            token(TokenKind::Ident("Box".into())),
            token(TokenKind::Colon),
            token(TokenKind::LineBreak),
            token(TokenKind::Ident("height".into())),
            token(TokenKind::Equal),
            token(TokenKind::Literal("5mm".into())),
        ];
        assert_eq!(
            separate_tokens_by_statement(tokens.clone()),
            vec![
                vec![
                    token(TokenKind::Ident("part".into())),
                    token(TokenKind::Ident("Box".into())),
                    token(TokenKind::Colon),
                ],
                vec![
                    token(TokenKind::Ident("height".into())),
                    token(TokenKind::Equal),
                    token(TokenKind::Literal("5mm".into())),
                ]
            ]
        )
    }

    #[test]
    fn two_assignments() {
        let tokens = vec![
            token(TokenKind::Ident("width".into())),
            token(TokenKind::Equal),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::LineBreak),
            token(TokenKind::Ident("height".into())),
            token(TokenKind::Equal),
            token(TokenKind::Literal("6mm".into())),
        ];
        assert_eq!(
            separate_tokens_by_statement(tokens.clone()),
            vec![
                vec![
                    token(TokenKind::Ident("width".into())),
                    token(TokenKind::Equal),
                    token(TokenKind::Literal("5mm".into())),
                ],
                vec![
                    token(TokenKind::Ident("height".into())),
                    token(TokenKind::Equal),
                    token(TokenKind::Literal("6mm".into())),
                ]
            ]
        )
    }

    #[test]
    fn two_expressions_separated_by_linebreak() {
        let tokens = vec![
            token(TokenKind::Ident("Rectangle".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::Comma),
            token(TokenKind::Literal("6mm".into())),
            token(TokenKind::RParen),
            token(TokenKind::LineBreak),
            token(TokenKind::Ident("Sphere".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("7mm".into())),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            separate_tokens_by_statement(tokens.clone()),
            vec![
                vec![
                    token(TokenKind::Ident("Rectangle".into())),
                    token(TokenKind::LParen),
                    token(TokenKind::Literal("5mm".into())),
                    token(TokenKind::Comma),
                    token(TokenKind::Literal("6mm".into())),
                    token(TokenKind::RParen),
                ],
                vec![
                    token(TokenKind::Ident("Sphere".into())),
                    token(TokenKind::LParen),
                    token(TokenKind::Literal("7mm".into())),
                    token(TokenKind::RParen),
                ]
            ]
        )
    }

    #[test]
    fn linebreaks_inside_expression() {
        let tokens = vec![
            token(TokenKind::Ident("Sphere".into())),
            token(TokenKind::LParen),
            token(TokenKind::LineBreak),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::LineBreak),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            separate_tokens_by_statement(tokens.clone()),
            vec![vec![
                token(TokenKind::Ident("Sphere".into())),
                token(TokenKind::LParen),
                token(TokenKind::Literal("5mm".into())),
                token(TokenKind::RParen),
            ],]
        )
    }

    #[test]
    fn dot_after_linebreak() {
        let tokens = vec![
            token(TokenKind::Ident("Sphere".into())),
            token(TokenKind::LParen),
            token(TokenKind::Literal("5mm".into())),
            token(TokenKind::RParen),
            token(TokenKind::LineBreak),
            token(TokenKind::Dot),
            token(TokenKind::Ident("add".into())),
            token(TokenKind::LParen),
            token(TokenKind::RParen),
        ];
        assert_eq!(
            separate_tokens_by_statement(tokens.clone()),
            vec![vec![
                token(TokenKind::Ident("Sphere".into())),
                token(TokenKind::LParen),
                token(TokenKind::Literal("5mm".into())),
                token(TokenKind::RParen),
                token(TokenKind::Dot),
                token(TokenKind::Ident("add".into())),
                token(TokenKind::LParen),
                token(TokenKind::RParen),
            ],]
        )
    }
}

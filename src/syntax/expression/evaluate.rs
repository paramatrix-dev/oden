use super::{ExprKind, Expression};
use crate::{Member, PartNamespace, errors::Error};

impl Expression {
    /// Evaluate an expression to a Member.
    pub fn evaluate(&self, namespace: &PartNamespace) -> Result<Member, Error> {
        let span = self.span().clone();
        match self.kind() {
            ExprKind::Literal(val) => Member::from_str(val, span),
            ExprKind::Ident(key) => match namespace.get(key) {
                Some(val) => Ok(val.clone()),
                None => Err(Error::UnknownVariable(key.to_owned(), span)),
            },
            ExprKind::Function { name, args } => match namespace.get(name) {
                Some(Member::Type(t)) => t.call(&eval_args(args, namespace)?, span),
                _ => Err(Error::UnknownFunction(name.to_owned(), span)),
            },
            ExprKind::Method {
                receiver,
                method,
                args,
            } => receiver.evaluate(namespace)?.method(
                method.into(),
                &eval_args(args, namespace)?,
                &span,
            ),
        }
    }
}

fn eval_args(args: &Vec<Expression>, namespace: &PartNamespace) -> Result<Vec<Member>, Error> {
    let mut evaluated = vec![];

    for arg in args {
        evaluated.push(arg.evaluate(namespace)?);
    }

    Ok(evaluated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Span;
    use anvil::{Cuboid, Length};

    fn expr_literal(value: &str) -> Expression {
        Expression(ExprKind::Literal(value.into()), Span::empty())
    }

    fn length_val(mm: f64) -> Member {
        Member::Instance(Box::new(Length::from_mm(mm)))
    }

    #[test]
    fn numeric() {
        let expression = Expression(ExprKind::Literal("3.15".into()), Span::empty());
        let namespace = PartNamespace::new();

        assert_eq!(
            expression.evaluate(&namespace),
            Ok(Member::Instance(Box::new(3.15))),
        )
    }

    #[test]
    fn length() {
        let expression = Expression(ExprKind::Literal("5mm".into()), Span::empty());
        let namespace = PartNamespace::new();

        assert_eq!(
            expression.evaluate(&namespace),
            Ok(Member::Instance(Box::new(Length::from_mm(5.)))),
        )
    }

    #[test]
    fn identity() {
        let expression = Expression(ExprKind::Ident("height".into()), Span::empty());
        let namespace = PartNamespace::new().insert_clone("height".into(), length_val(5.));

        assert_eq!(expression.evaluate(&namespace), Ok(length_val(5.)),)
    }

    #[test]
    fn cuboid_function() {
        let expression = Expression(
            ExprKind::Function {
                name: "Cuboid".into(),
                args: vec![
                    expr_literal("5mm"),
                    expr_literal("6mm"),
                    expr_literal("7mm"),
                ],
            },
            Span::empty(),
        );
        let namespace = PartNamespace::new();

        assert_eq!(
            expression.evaluate(&namespace),
            Ok(Member::Instance(Box::new(Cuboid::from_mm(5., 6., 7.)))),
        )
    }

    #[test]
    fn method() {
        let expression = Expression(
            ExprKind::Method {
                receiver: Box::new(Expression(
                    ExprKind::Function {
                        name: "Cuboid".into(),
                        args: vec![
                            expr_literal("1mm"),
                            expr_literal("1mm"),
                            expr_literal("5mm"),
                        ],
                    },
                    Span::empty(),
                )),
                method: "add".into(),
                args: [Expression(
                    ExprKind::Function {
                        name: "Cuboid".into(),
                        args: vec![
                            expr_literal("5mm"),
                            expr_literal("1mm"),
                            expr_literal("1mm"),
                        ],
                    },
                    Span::empty(),
                )]
                .into(),
            },
            Span::empty(),
        );
        let namespace = PartNamespace::new();

        assert_eq!(
            expression.evaluate(&namespace),
            Ok(Member::Instance(Box::new(
                Cuboid::from_mm(1., 1., 5.).add(&Cuboid::from_mm(5., 1., 1.))
            ))),
        )
    }
}

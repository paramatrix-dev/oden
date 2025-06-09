use super::_struct::{Statement, StmtKind};
use crate::{
    PartNamespace,
    errors::Error,
    syntax::expression::{ExprKind, Expression},
};

impl Statement {
    /// Apply the statement to a given namespace.
    pub fn execute(self, namespace: &mut PartNamespace) -> Result<(), Error> {
        match self.0 {
            StmtKind::Assignment(name, expr) => {
                namespace.insert(name, expr.evaluate(namespace)?);
            }
            StmtKind::Empty => (),
            StmtKind::Expr(expr) => match expr.kind().clone() {
                ExprKind::Method {
                    receiver,
                    method: _,
                    args: _,
                } => {
                    namespace.insert(resolve_receiver(*receiver)?, expr.evaluate(namespace)?);
                }
                _ => todo!(),
            },
            StmtKind::PartDeclaration(_) => (),
        }
        Ok(())
    }
}

fn resolve_receiver(receiver: Expression) -> Result<String, Error> {
    match receiver.kind().clone() {
        ExprKind::Ident(name) => Ok(name),
        ExprKind::Method {
            receiver,
            method: _,
            args: _,
        } => resolve_receiver(*receiver),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        syntax::{Span, expression::ExprKind},
        values::Member,
    };
    use anvil::{Cuboid, IntoLength, Length};

    #[test]
    fn part_declaration() {
        let statement = Statement(StmtKind::PartDeclaration("Box".into()), Span::empty());
        let mut namespace = PartNamespace::new();

        assert!(statement.execute(&mut namespace).is_ok());
        assert_eq!(namespace, PartNamespace::new()) // nothing happened
    }

    #[test]
    fn assignment() {
        let statement = Statement(
            StmtKind::Assignment("height".into(), Expression::lit("5mm")),
            Span::empty(),
        );
        let mut namespace = PartNamespace::new();

        assert!(statement.execute(&mut namespace).is_ok());
        assert_eq!(
            namespace.get(&"height".into()),
            Some(&Member::Length(Length::from_mm(5.)))
        )
    }

    #[test]
    fn part_add_expression() {
        let statement = Statement(
            StmtKind::Expr(Expression(
                ExprKind::Method {
                    receiver: Box::new(Expression::ident("part")),
                    method: "add".into(),
                    args: vec![Expression::ident("box")],
                },
                Span::empty(),
            )),
            Span::empty(),
        );
        let mut namespace = PartNamespace::new().insert_clone(
            "box".into(),
            Member::Part(Cuboid::from_dim(5.mm(), 6.mm(), 7.mm())),
        );

        assert!(statement.execute(&mut namespace).is_ok());
        assert_eq!(namespace.part(), Cuboid::from_dim(5.mm(), 6.mm(), 7.mm(),))
    }
}

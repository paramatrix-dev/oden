use anvil::Part;

use crate::{
    PartNamespace,
    errors::Error,
    syntax::{Statement, separate_tokens_by_statement, tokenize},
};

/// Compile an oden part that is loaded into a &str.
///
/// The file argument is only used to construct the Span, so this does not necessarily have to be a
/// valid path.
pub fn compile_input(input: &str) -> Result<Part, Error> {
    let mut namespace = PartNamespace::new();

    let tokens = tokenize(input)?;

    let statements = separate_tokens_by_statement(tokens)
        .iter()
        .map(Statement::from_tokens)
        .collect::<Result<Vec<Statement>, Error>>()?;

    for statement in statements {
        statement.execute(&mut namespace)?;
    }

    Ok(namespace.part())
}

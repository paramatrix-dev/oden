use std::path::PathBuf;

mod compile;
mod errors;
mod namespace;
mod syntax;
mod values;

pub use compile::compile_input;
pub use errors::Error;
pub use namespace::PartNamespace;
pub use syntax::{ExprKind, Expression, Span, Statement, Token, TokenKind, eval_str, tokenize};
pub use values::{
    Value,
    axis::AxisType,
    path::PathType,
    traits::{Instance, Type, check_args},
};

/// Compile an oden file and write the resulting shape into an STEP file.
pub fn compile(source: PathBuf, target: PathBuf) -> Result<(), Error> {
    use std::fs;

    let input = match fs::read_to_string(source.clone()) {
        Ok(text) => text,
        Err(_) => return Err(Error::FileNotFound(source.clone())),
    };

    let part = compile_input(&input, source)?;
    Error::from_anvil(part.write_step(target), None)
}

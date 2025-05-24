pub mod compile;
pub mod errors;
pub mod namespaces;
pub mod syntax;
pub mod values;

use compile::compile_input;
use errors::Error;
use std::path::PathBuf;
pub use values::Value;

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

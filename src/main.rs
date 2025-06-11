use clap::Parser;
use std::path::PathBuf;

use oden::{Error, compile};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the oden file.
    #[arg(short, long)]
    source: PathBuf,

    /// Target path of the STL file.
    #[arg(short, long)]
    target: PathBuf,
}

/// Compile an oden file and write the resulting shape into an STEP file.
fn main() -> Result<(), Error> {
    let args = Args::parse();
    compile(args.source, args.target)
}

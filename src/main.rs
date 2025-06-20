use clap::Parser;
use std::path::PathBuf;

use oden::compile;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the oden file.
    #[arg(short, long)]
    source: PathBuf,

    /// Target path of the STL file.
    #[arg(short, long)]
    target: PathBuf,

    /// If set, no console ouput is produced
    #[arg(short, long, action)]
    quiet: bool,
}

/// Compile an oden file and write the resulting shape into an STEP file.
fn main() {
    let args = Args::parse();
    match (compile(args.source, args.target), args.quiet) {
        (Err(error), false) => println!("{}", error),
        _ => (),
    }
}

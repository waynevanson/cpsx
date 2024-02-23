use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
struct Args {
    source: PathBuf,
    target: PathBuf,
    prefix: PathBuf,
}

fn main() {
    let mut args = Args::parse();
}

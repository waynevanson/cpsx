use std::{fs::copy, path::PathBuf};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
struct Args {
    source: PathBuf,
    target: PathBuf,
    prefix: PathBuf,
}

fn main() {
    let mut args = Args::parse();

    let source = args.source;

    let suffix = source.clone();
    let suffix = suffix.strip_prefix(args.prefix).unwrap();

    args.target.push(&suffix);

    copy(source, args.target).unwrap();
}

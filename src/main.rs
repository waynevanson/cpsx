use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
struct Args {
    source: PathBuf,

    /// Defaults to the platform specific `$HOME/.dotfiles/`.
    #[arg()]
    store: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser, Clone)]
enum Commands {
    Apply,
    Update,
    Add {
        /// Path to the file or directory that should be saved to the dotfiles
        target: PathBuf,
        /// The Defaults to the store.
        sub_store: Option<PathBuf>,
    },
    Remove {
        /// Path to the file or directory that should be removes from the dotfiles
        target: PathBuf,
        /// Defaults to the store.
        sub_store: Option<PathBuf>,
    },
}

fn main() {
    let args = Args::parse();
    let store = args
        .store
        .or_else(|| home::home_dir())
        .expect("Unable to find the store directory automatically. Please explicitly set it by using the flag '--store' or '-s'");
}

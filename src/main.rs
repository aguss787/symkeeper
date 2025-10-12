mod config;
mod error;
mod fs;
mod path_ext;
mod prelude;
mod symlink;
mod sync;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short, long)]
    dry_run: bool,
    #[arg(short = 'f', long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Synchronize symlinks from config to filesystem
    Sync {
        /// Overwrite existing symlinks
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = inner(args) {
        println!("Error happened during execution");
        println!("{}", err);
    }
}

fn inner(args: CliArgs) -> Result<()> {
    let config = config::Config::load(args.config)?;
    let fs = fs::Fs::new(args.dry_run);
    match args.command {
        Command::Sync { force } => {
            sync::SyncRunner::new(fs, force).run(config)?;
        }
    }

    Ok(())
}

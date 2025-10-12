mod config;
mod error;
mod fs;
mod path_ext;
mod prelude;
mod symlink;
mod sync;

use clap::{Parser, Subcommand};

use crate::prelude::*;

#[derive(Debug, Parser)]
struct CliArgs {
    #[arg(short, long)]
    dry_run: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Sync,
}

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = inner(args) {
        println!("Error happened during execution");
        println!("{}", err);
    }
}

fn inner(args: CliArgs) -> Result<()> {
    let config = config::Config::load()?;
    let fs = fs::Fs::new(args.dry_run);
    match args.command {
        Command::Sync => sync::run(fs, config)?,
    }

    Ok(())
}

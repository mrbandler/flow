use clap::Subcommand;
use miette::Result;

use crate::commands::{init, Command};

mod commands;
mod common;
mod errors;
mod extensions;
mod printer;

#[derive(Subcommand)]
pub enum Commands {
    Init(init::Arguments),
}

/// Run the CLI with the given command.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub async fn run(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::Init(args) => init::Init::new(args.clone()).run().await?,
    }

    Ok(())
}

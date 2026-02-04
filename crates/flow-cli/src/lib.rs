//! Command-line interface for Flow.
//!
//! This crate provides the CLI commands for Flow, including space
//! initialization, management, and note operations. Commands support
//! both interactive and non-interactive (JSON output) modes.
//!
//! # Architecture
//!
//! Commands implement the `Command` trait, which provides a consistent
//! pattern for:
//!
//! - Interactive prompting when arguments are missing
//! - Execution logic
//! - Output formatting (human-readable or JSON)
//!
//! # Usage
//!
//! The main entry point is the [`run`] function, which dispatches to
//! the appropriate command handler based on the [`Commands`] enum.
//!
//! ```ignore
//! use flow_cli::{Commands, run};
//! use flow_cli::commands::space;
//!
//! let cmd = Commands::Space(space::Commands::Init(space::init::Arguments { /* ... */ }));
//! run(cmd).await?;
//! ```

use clap::Subcommand;
use miette::Result;

use crate::{commands::space, context::Context};

mod commands;
mod common;
mod context;
mod printer;
mod theme;
mod validators;

/// Available CLI commands.
///
/// Each variant corresponds to a subcommand that can be invoked from
/// the command line (e.g., `flow space init`).
#[derive(Subcommand)]
pub enum Commands {
    /// Manage Flow spaces.
    #[command(subcommand)]
    Space(space::Space),
}

/// Run the CLI with the given command.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub async fn run(cmd: Commands) -> Result<()> {
    let mut ctx = Context::load().await?;

    match cmd {
        Commands::Space(space) => space.run(&mut ctx).await?,
    }

    Ok(())
}

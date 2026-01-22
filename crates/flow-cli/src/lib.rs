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
//!
//! let cmd = Commands::Init(init::Arguments { /* ... */ });
//! run(cmd).await?;
//! ```

use clap::Subcommand;
use miette::Result;

use crate::commands::{init, Command};

mod commands;
mod common;
mod printer;

/// Available CLI commands.
///
/// Each variant corresponds to a subcommand that can be invoked from
/// the command line (e.g., `flow init`).
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Flow space.
    Init(init::Arguments),
}

/// Run the CLI with the given command.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub async fn run(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Init(args) => init::Init::new(args).run().await?,
    }

    Ok(())
}

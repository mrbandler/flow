//! Flow CLI - Command-line interface for Flow note-taking system.
//!
//! This crate provides the CLI commands for Flow. Each command is defined
//! in its own module under `commands/` for better organization and maintainability.
//!
//! # Documentation
//!
//! For detailed documentation on the CLI architecture, how to add new commands,
//! and best practices, see the [README](../README.md) in this crate's directory.
//!
//! ## Quick Overview
//!
//! - **Commands**: Each command lives in `commands/` and implements the `Command` trait
//! - **Global Flags**: All commands support `--json`, `--graph`, `--verbose`, `--quiet` via `GlobalArgs`
//! - **Output Handling**: Commands handle their own output using `GlobalArgs` helper methods
//! - **Error Handling**: Commands return `Result<()>` - errors bubble up to the main binary

pub mod commands;
pub mod common;
pub mod error;

use clap::Subcommand;
use miette::Result;

use crate::common::Command;

/// CLI commands.
///
/// Each variant corresponds to a subcommand that can be invoked from the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Flow graph
    Init(commands::init::InitArgs),

    /// Open an existing graph (by name or path)
    Open(commands::open::OpenArgs),

    /// Add a node to today's journal page
    Add(commands::add::AddArgs),

    /// Remove orphaned graphs from configuration
    Clean(commands::clean::CleanArgs),
}

/// Runs the CLI command.
///
/// This function dispatches to the appropriate command handler based on
/// the parsed command-line arguments.
///
/// # Arguments
///
/// * `cmd` - The parsed command to execute
///
/// # Returns
///
/// * `Result<()>` - Success or error from the command execution
///
/// # Errors
///
/// Returns an error if the command execution fails. Each command module
/// documents its specific error conditions.
pub fn run(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Init(args) => commands::init::InitCommand::from_args(args).execute(),
        Commands::Open(args) => commands::open::OpenCommand::from_args(args).execute(),
        Commands::Add(args) => commands::add::AddCommand::from_args(args).execute(),
        Commands::Clean(args) => commands::clean::CleanCommand::from_args(args).execute(),
    }
}

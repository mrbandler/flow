//! Common argument types shared across CLI commands.
//!
//! This module provides reusable argument structs that can be embedded
//! in command-specific argument types using clap's `#[command(flatten)]`.

use clap::Args;
use flow_core::Locator;

/// Output formatting arguments available for all commands.
///
/// These arguments control how command output is displayed and can be
/// embedded in any command's arguments using `#[command(flatten)]`.
#[derive(Args, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct GlobalArgs {
    /// Force interactive mode, prompting for all arguments.
    #[arg(short, long, global = true)]
    pub interactive: bool,

    /// Output in JSON format.
    #[arg(long, global = true)]
    pub json: bool,

    /// Enable detailed logging output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable trace-level logging output (implies --verbose).
    #[arg(short, long, global = true)]
    pub trace: bool,

    /// Suppress non-error output.
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

/// Arguments for commands that operate on an existing space.
///
/// This struct combines output formatting with space selection, providing
/// a consistent way to target a specific space or use the active default.
#[derive(Args, Debug, Clone)]
pub struct SpaceArgs {
    /// Output formatting options.
    #[command(flatten)]
    pub output: GlobalArgs,

    /// Target a specific space by name or path, overriding the active space.
    #[arg(long)]
    pub space: Option<Locator>,
}

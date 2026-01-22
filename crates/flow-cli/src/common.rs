//! Common argument types shared across CLI commands.
//!
//! This module provides reusable argument structs that can be embedded
//! in command-specific argument types using clap's `#[command(flatten)]`.

use clap::Args;
use flow_core::{Config, Space};
use miette::Result;
use std::path::PathBuf;

use crate::printer::Printer;
use flow_errors::CliError;

/// Output formatting arguments available for all commands.
///
/// These arguments control how command output is displayed and can be
/// embedded in any command's arguments using `#[command(flatten)]`.
#[derive(Args, Debug, Clone)]
pub struct OutputArgs {
    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Detailed logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

impl OutputArgs {
    /// Create a printer from these output arguments.
    #[must_use]
    pub const fn printer(&self) -> Printer {
        Printer::new(self.json, self.verbose, self.quiet)
    }
}

/// Arguments for commands that operate on an existing space.
///
/// This struct combines output formatting with space selection, providing
/// a consistent way to target a specific space or use the active default.
#[derive(Args, Debug, Clone)]
pub struct SpaceArgs {
    /// Output formatting options.
    #[command(flatten)]
    pub output: OutputArgs,

    /// Target a specific space by name or path, overriding the active space.
    #[arg(long)]
    pub space: Option<String>,
}

impl SpaceArgs {
    /// Creates a printer configured with these arguments' output settings.
    #[must_use]
    #[allow(dead_code)]
    pub const fn printer(&self) -> Printer {
        self.output.printer()
    }

    /// Loads the target space based on these arguments.
    ///
    /// # Resolution Order
    ///
    /// 1. If `--space` is a valid filesystem path, load from that path
    /// 2. If `--space` matches a registered space name, load that space
    /// 3. Otherwise, load the currently active space from config
    ///
    /// # Errors
    ///
    /// Returns [`CliError::NoActiveSpace`] if no space can be determined,
    /// or a space loading error if the space cannot be read.
    #[allow(dead_code)]
    pub async fn load_space(&self) -> Result<Space> {
        if let Some(name_or_path) = &self.space {
            let path = PathBuf::from(name_or_path);
            if path.exists() {
                return Space::load(path).await;
            }
        }

        let config = Config::load().await?;
        let space = self
            .space
            .as_ref()
            .and_then(|n| config.find(n))
            .or_else(|| config.active())
            .ok_or(CliError::NoActiveSpace)?;

        Space::load(&space.name).await
    }
}

use clap::Args;
use flow_core::{Config, Space};
use miette::Result;
use std::path::PathBuf;

use crate::{errors::Error, printer::Printer};

/// Output formatting arguments - available for ALL commands.
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

/// Arguments for commands that operate on existing spaces.
#[derive(Args, Debug, Clone)]
pub struct SpaceArgs {
    /// Embedded output arguments
    #[command(flatten)]
    pub output: OutputArgs,

    /// Target specific space by name or path (overrides active space)
    #[arg(long)]
    pub space: Option<String>,
}

impl SpaceArgs {
    /// Create a printer from these space arguments.
    #[must_use]
    #[allow(dead_code)]
    pub const fn printer(&self) -> Printer {
        self.output.printer()
    }

    /// Load the space based on the provided arguments.
    ///
    /// Resolution order:
    /// 1. If `--space` is provided and is a valid path, load from that path
    /// 2. If `--space` is provided and matches a registered space name, load that space
    /// 3. Otherwise, load the currently active space from config
    ///
    /// # Errors
    ///
    /// Returns an error if no space can be found or loaded.
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
            .ok_or(Error::NoActiveSpace)?;

        Space::load(&space.name).await
    }
}

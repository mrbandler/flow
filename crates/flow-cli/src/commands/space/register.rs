//! The `register` command for registering existing Flow spaces.
//!
//! This module implements the `flow space register` command, which registers
//! an existing (already initialized) Flow space in the global configuration.
//! This allows the space to appear in `flow space list` and be switched to by name.
//!
//! # Examples
//!
//! ```bash
//! # Interactive mode - prompts for path
//! flow space register
//!
//! # Non-interactive with path
//! flow space register ./my-notes
//!
//! # JSON output for scripting
//! flow space register ./my-notes --json
//! ```

use std::path::PathBuf;

use clap::Args;
use flow_common::PathExt;
use flow_core::Space;
use flow_errors::CliError;
use inquire::Text;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

use crate::{commands::Command, common::GlobalArgs, context::Context};

/// Command-line arguments for the `register` command.
#[derive(Args, Debug, Clone)]
pub struct Arguments {
    /// Global arguments
    #[command(flatten)]
    pub globals: GlobalArgs,

    /// Path to the existing space to register
    pub path: Option<PathBuf>,
}

/// Output of a successful `register` command.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// The name of the registered space.
    pub name: String,

    /// The filesystem path where the space is located.
    pub path: PathBuf,
}

/// The `register` command implementation.
pub struct Register<'a> {
    args: Arguments,
    ctx: &'a mut Context,
}

impl<'a> Command<'a> for Register<'a> {
    type Args = Arguments;
    type Output = Output;

    fn new(args: Self::Args, ctx: &'a mut Context) -> Self {
        Self { args, ctx }
    }

    fn ctx(&self) -> &Context {
        self.ctx
    }

    fn globals(&self) -> &GlobalArgs {
        &self.args.globals
    }

    fn needs_interaction(&self) -> bool {
        self.globals().interactive || self.args.path.is_none()
    }

    async fn interactive(&mut self) -> Result<()> {
        let printer = self.printer();
        printer.info("Entering interactive mode");

        let forced = self.globals().interactive;

        if forced || self.args.path.is_none() {
            let default = self
                .args
                .path
                .as_ref()
                .map_or_else(|| ".".to_string(), |p| p.normalize_to_string());

            let path_input = Text::new("Path:")
                .with_default(&default)
                .with_help_message("Path to the existing space to register")
                .prompt()
                .into_diagnostic()?;

            self.args.path = Some(PathBuf::from(path_input));
        }

        Ok(())
    }

    async fn execute(&mut self) -> Result<Self::Output> {
        let path = self
            .args
            .path
            .take()
            .ok_or_else(|| CliError::MissingArgument("path".to_string()))?;

        let space = Space::load(path).await?;
        let name = space.name().to_string();

        let config = self.ctx.config_mut();
        config.register(&space).await?;
        if config.active().is_none() {
            config.set_active(&name.as_str().into()).await?;
        }

        Ok(Output {
            name,
            path: space.path().to_path_buf(),
        })
    }

    fn finalize(&self, output: &Self::Output) {
        let printer = self.printer();

        printer.success("Space registered:");
        printer.kv("Name", &output.name);
        printer.kv("Path", output.path.normalize_to_string());
    }
}

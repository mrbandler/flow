//! The `init` command for creating new Flow spaces.
//!
//! This module implements the `flow space init` command, which initializes a new
//! Flow space at a specified path. The command supports both interactive
//! and non-interactive modes.
//!
//! # Examples
//!
//! ```bash
//! # Interactive mode - prompts for path and name
//! flow space init
//!
//! # Non-interactive with arguments
//! flow space init ./my-notes --name personal
//!
//! # JSON output for scripting
//! flow space init ./my-notes --name personal --json
//! ```

use std::path::PathBuf;

use clap::Args;
use flow_common::PathExt;
use flow_core::Space;
use flow_errors::CliError;
use inquire::{Confirm, Text};
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::{
    commands::Command,
    common::GlobalArgs,
    context::Context,
    validators::{NameAlreadyRegisteredValidator, PathAlreadyExistsValidator},
};

/// Command-line arguments for the `init` command.
#[derive(Args, Debug, Clone)]
pub struct Arguments {
    /// Global arguments
    #[command(flatten)]
    pub globals: GlobalArgs,

    /// Path to initialize the space at
    pub path: Option<PathBuf>,

    /// Name of the space (defaults to the directory name)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Flag, to not register the space
    #[arg(long)]
    pub no_register: bool,
}

/// Output of a successful `init` command.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// The name assigned to the space.
    pub name: String,

    /// The filesystem path where the space was created.
    pub path: PathBuf,
}

/// The `init` command implementation.
pub struct Init<'a> {
    args: Arguments,
    ctx: &'a mut Context,
}

impl<'a> Command<'a> for Init<'a> {
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
        self.globals().interactive || self.args.path.is_none() || self.args.name.is_none()
    }

    async fn interactive(&mut self) -> miette::Result<()> {
        let forced = self.globals().interactive;

        // When 'forced', we ask whether the user wants to override default values of non-required arguments.
        let prompt_overrides = forced
            && Confirm::new("Do you want to override default flags?")
                .with_default(false)
                .prompt()
                .into_diagnostic()?;

        if prompt_overrides {
            self.args.no_register = !Confirm::new("Register this space?")
                .with_default(!self.args.no_register)
                .with_help_message("Registered spaces appear in 'flow space list' and can be switched to by name")
                .prompt()
                .into_diagnostic()?;
        }

        if forced || self.args.path.is_none() {
            let default = self
                .args
                .path
                .as_ref()
                .map_or_else(|| ".".to_string(), |p| p.normalize_to_string());

            let path_input = Text::new("Path:")
                .with_default(&default)
                .with_help_message("Path where the space will be initialized")
                .with_validator(PathAlreadyExistsValidator::new())
                .prompt()
                .into_diagnostic()?;

            self.args.path = Some(PathBuf::from(path_input));
        }

        if forced || self.args.name.is_none() {
            let default = self.args.name.clone().or_else(|| {
                self.args
                    .path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .map(String::from)
            });

            let mut prompt = Text::new("Name:").with_help_message("Name of the space");
            if !self.args.no_register {
                prompt = prompt.with_validator(NameAlreadyRegisteredValidator::new(self.ctx.config()));
            }

            if let Some(d) = &default {
                prompt = prompt.with_default(d);
            }

            let name_input = prompt.prompt().into_diagnostic()?;
            if !name_input.trim().is_empty() {
                self.args.name = Some(name_input);
            }
        }

        Ok(())
    }

    async fn execute(&mut self) -> miette::Result<Self::Output> {
        let path = self
            .args
            .path
            .take()
            .ok_or_else(|| CliError::MissingArgument("path".to_string()))?;

        let path_name = path.file_name().and_then(|n| n.to_str());
        let name = self
            .args
            .name
            .take()
            .or_else(|| path_name.map(String::from))
            .ok_or_else(|| CliError::MissingArgument("name".to_string()))?;

        let space = Space::init(&path, &name).await?;
        if !self.args.no_register {
            let config = self.ctx.config_mut();
            config.register(&space).await?;
            if config.active().is_none() {
                config.set_active(&space.name().into()).await?;
            }
        }

        Ok(Output { name, path })
    }

    fn finalize(&self, _output: &Self::Output) {
        self.printer().success("Space initialized");
    }
}

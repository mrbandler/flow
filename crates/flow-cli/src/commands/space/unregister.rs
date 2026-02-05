//! The `unregister` command for removing spaces from Flow's configuration.
//!
//! This module implements the `flow space unregister` command, which removes
//! a space from Flow's registry. The space will no longer appear in
//! `flow space list` and cannot be switched to by name.
//!
//! Optionally, the `--delete` flag can be used to also delete the space's
//! files from disk.
//!
//! # Examples
//!
//! ```bash
//! # Interactive mode - select space to unregister
//! flow space unregister
//!
//! # Unregister by name
//! flow space unregister personal
//!
//! # Unregister by path
//! flow space unregister ./my-notes
//!
//! # Unregister and delete files from disk
//! flow space unregister personal --delete
//!
//! # JSON output for scripting
//! flow space unregister personal --json
//! ```

use std::path::PathBuf;

use clap::Args;
use flow_common::PathExt;
use flow_core::{Locator, SpaceError};
use flow_errors::CliError;
use inquire::{Confirm, Select};
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

use crate::{
    commands::{space::SpaceOption, Command},
    common::GlobalArgs,
    context::Context,
};

/// Command-line arguments for the `unregister` command.
#[derive(Args, Debug, Clone)]
pub struct Arguments {
    /// Global arguments.
    #[command(flatten)]
    pub globals: GlobalArgs,

    /// The locator (name or path) of the space to unregister.
    pub locator: Option<Locator>,

    /// Whether to delete the space's files from disk.
    #[arg(long, short)]
    pub delete: bool,
}

/// Output of a successful `unregister` command.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// The name of the unregistered space.
    pub name: String,

    /// The filesystem path where the space was located.
    pub path: PathBuf,

    /// Whether the space's files were deleted from disk.
    pub delete: bool,
}

/// Unregisters a Flow space from the configuration.
///
/// This command removes a space from Flow's registry. Optionally,
/// the space's files can be deleted from disk with the `--delete` flag.
pub struct Unregister<'a> {
    args: Arguments,
    ctx: &'a mut Context,
}

impl<'a> Command<'a> for Unregister<'a> {
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
        self.globals().interactive || self.args.locator.is_none()
    }

    async fn interactive(&mut self) -> Result<()> {
        let forced = self.globals().interactive;

        // When 'forced', we ask whether the user wants to override default values of non-required arguments.
        let prompt_overrides = forced
            && Confirm::new("Do you want to override default flags?")
                .with_default(false)
                .prompt()
                .into_diagnostic()?;

        if prompt_overrides {
            self.args.delete = Confirm::new("Delete selected space?")
                .with_default(self.args.delete)
                .with_help_message("Will delete the space's files from disk")
                .prompt()
                .into_diagnostic()?;
        }

        if forced || self.args.locator.is_none() {
            let cwd_locator;
            let default_locator = if let Some(loc) = &self.args.locator {
                loc
            } else {
                let cwd = std::env::current_dir().into_diagnostic()?;
                cwd_locator = Locator::from(cwd.normalize());
                &cwd_locator
            };

            let (options, default_index) = SpaceOption::from_context(self.ctx, Some(default_locator));
            let selected = Select::new("Select the space to unregister:", options)
                .with_starting_cursor(default_index)
                .prompt()
                .into_diagnostic()?;

            self.args.locator = Some(selected.name.into());
        }

        Ok(())
    }

    async fn execute(&mut self) -> Result<Self::Output> {
        let locator = self
            .args
            .locator
            .take()
            .ok_or_else(|| CliError::MissingArgument("locator".to_string()))?;

        let config = self.ctx.config_mut();
        let space = config
            .find(&locator)
            .await
            .ok_or_else(|| SpaceError::NotRegistered(locator.to_string()))?;

        let name = space.name.clone();
        let path = space.path.clone();

        config.unregister(&locator, self.args.delete).await?;

        Ok(Output {
            name,
            path,
            delete: self.args.delete,
        })
    }

    fn finalize(&self, output: &Self::Output) {
        let msg = if output.delete { " and deleted from disk" } else { "" };

        self.printer().success(format!("Space unregistered{msg}"));
    }
}

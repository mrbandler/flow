//! The `switch` command for changing the active Flow space.
//!
//! This module implements the `flow space switch` command, which sets
//! a different registered space as the active space. The active space
//! is the default space used when no space is explicitly specified.
//!
//! # Examples
//!
//! ```bash
//! # Interactive mode - select from registered spaces
//! flow space switch
//!
//! # Switch by name
//! flow space switch personal
//!
//! # Switch by path
//! flow space switch ./my-notes
//!
//! # JSON output for scripting
//! flow space switch personal --json
//! ```

use std::path::PathBuf;

use clap::Args;
use flow_common::PathExt;
use flow_core::Locator;
use flow_errors::CliError;
use inquire::Select;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

use crate::{
    commands::{space::SpaceOption, Command},
    common::GlobalArgs,
    context::Context,
};

/// Command-line arguments for the `switch` command.
#[derive(Args, Debug, Clone)]
pub struct Arguments {
    /// Global arguments.
    #[command(flatten)]
    pub globals: GlobalArgs,

    /// The locator (name or path) of the space to switch to.
    pub locator: Option<Locator>,
}

/// Output of a successful `switch` command.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// The name of the newly active space.
    pub name: String,

    /// The filesystem path to the newly active space.
    pub path: PathBuf,
}

/// Switches the active Flow space.
///
/// This command changes which space is used by default when no space
/// is explicitly specified in other commands.
pub struct Switch<'a> {
    args: Arguments,
    ctx: &'a mut Context,
}

impl<'a> Command<'a> for Switch<'a> {
    type Args = Arguments;
    type Output = Output;

    fn new(args: Self::Args, ctx: &'a mut Context) -> Self {
        Self { args, ctx }
    }

    fn ctx(&self) -> &Context {
        self.ctx
    }

    fn globals(&self) -> &crate::common::GlobalArgs {
        &self.args.globals
    }

    async fn validate(&mut self) -> Result<()> {
        if self.ctx.config().spaces().is_empty() {
            return Err(CliError::NoSpacesRegistered.into());
        }

        Ok(())
    }

    fn needs_interaction(&self) -> bool {
        self.globals().interactive || self.args.locator.is_none()
    }

    async fn interactive(&mut self) -> Result<()> {
        let printer = self.printer();
        printer.info("Entering interactive mode");

        let forced = self.globals().interactive;

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
            let selected = Select::new("Select the space to switch to:", options)
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
        config.set_active(&locator).await?;

        let active = config.active().ok_or(CliError::NoActiveSpace)?;

        Ok(Output {
            name: active.name.clone(),
            path: active.path.clone(),
        })
    }

    fn finalize(&self, _output: &Self::Output) {
        self.printer().success("Active space switched");
    }
}

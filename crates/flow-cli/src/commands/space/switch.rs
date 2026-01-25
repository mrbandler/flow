use std::{fmt, path::PathBuf};

use clap::Args;
use flow_common::PathExt;
use flow_core::{Config, Locator};
use flow_errors::CliError;
use inquire::Select;
use lazyinit::LazyInit;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

use crate::{commands::Command, common::OutputArgs};

/// A space option for interactive selection.
///
/// Displays as "name (path)" or "name (path) (active)" for the currently active space.
struct SpaceOption {
    name: String,
    path: PathBuf,
    is_active: bool,
}

impl fmt::Display for SpaceOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_active {
            write!(f, "{} ({}) ← active", self.name, self.path.normalize())
        } else {
            write!(f, "{} ({})", self.name, self.path.normalize())
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct Arguments {
    #[command(flatten)]
    pub output: OutputArgs,

    /// The locator (name or path) of the space to switch to.
    pub locator: Option<Locator>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// The name assigned to the space.
    pub name: String,
    /// The filesystem path where the space was created.
    pub path: PathBuf,
}

pub struct Switch {
    args: Arguments,
    config: LazyInit<Config>,
}

impl Command for Switch {
    type Args = Arguments;
    type Output = Output;

    fn new(args: Self::Args) -> Self {
        Self {
            args,
            config: LazyInit::new(),
        }
    }

    fn output_args(&self) -> &crate::common::OutputArgs {
        &self.args.output
    }

    async fn init(&mut self) -> Result<()> {
        self.config.init_once(Config::load().await?);
        Ok(())
    }

    async fn interactive(&mut self) -> Result<()> {
        if self.args.locator.is_none() {
            let active_name = self.config.active().map(|s| s.name.as_str());

            let options: Vec<SpaceOption> = self
                .config
                .spaces()
                .iter()
                .map(|space| SpaceOption {
                    name: space.name.clone(),
                    path: space.path.clone(),
                    is_active: active_name == Some(space.name.as_str()),
                })
                .collect();

            let selected = Select::new("Select the space to switch to:", options)
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

        self.config.set_active(locator).await?;

        let active = self.config.active().ok_or(CliError::NoActiveSpace)?;

        Ok(Output {
            name: active.name.clone(),
            path: active.path.clone(),
        })
    }

    fn finalize(&self, output: &Self::Output) {
        let printer = self.printer();

        printer.success("Active space switched to:");
        printer.kv("Name", &output.name);
        printer.kv("Path", output.path.normalize());
    }
}

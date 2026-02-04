//! The `list` command for displaying registered Flow spaces.
//!
//! This module implements the `flow space list` command, which shows all
//! registered spaces along with their paths. The currently active space
//! is marked with an asterisk (`*`).
//!
//! # Examples
//!
//! ```bash
//! # List all spaces in a table
//! flow space list
//!
//! # Output as JSON for scripting
//! flow space list --json
//! ```

use std::io::{stdout, Write};
use std::path::PathBuf;

use clap::Args;
use flow_common::PathExt;
use flow_errors::CliError;
use flow_theme::Theme as _;
use miette::Result;
use serde::Serialize;
use tabled::{
    settings::{
        object::{Cell, Rows},
        Color as TabledColor, Modify, Style,
    },
    Table, Tabled,
};

use crate::{commands::Command, common::GlobalArgs, context::Context, theme::HexColorExt};

/// Command-line arguments for the `list` command.
#[derive(Args, Debug, Clone)]
pub struct Arguments {
    #[command(flatten)]
    pub globals: GlobalArgs,
}

/// Output of a successful `list` command.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    /// All registered spaces.
    pub spaces: Vec<OutputSpace>,
}

/// A single space in the list output.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OutputSpace {
    /// The space's registered name.
    name: String,

    /// Path to the space's root directory.
    path: PathBuf,

    /// Whether this is the currently active space.
    active: bool,
}

/// Table row representation of a space for terminal display.
#[derive(Tabled)]
struct SpaceRow {
    /// Marker column (`*` for active space, empty otherwise).
    #[tabled(rename = "")]
    marker: &'static str,
    /// The space name.
    #[tabled(rename = "Name")]
    name: String,
    /// Normalized path to the space.
    #[tabled(rename = "Path")]
    path: String,
}

impl From<&OutputSpace> for SpaceRow {
    /// Converts an [`OutputSpace`] to a table row for display.
    fn from(space: &OutputSpace) -> Self {
        Self {
            marker: if space.active { "*" } else { "" },
            name: space.name.clone(),
            path: space.path.normalize_to_string(),
        }
    }
}

/// Lists all registered Flow spaces.
pub struct List<'a> {
    args: Arguments,
    ctx: &'a mut Context,
}

impl<'a> Command<'a> for List<'a> {
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

    async fn execute(&mut self) -> Result<Self::Output> {
        let config = self.ctx.config();
        let active_name = config.active().map(|s| &s.name);
        let spaces = config
            .spaces()
            .iter()
            .map(|space| OutputSpace {
                name: space.name.clone(),
                path: space.path.clone(),
                active: active_name == Some(&space.name),
            })
            .collect();

        Ok(Output { spaces })
    }

    fn finalize(&self, output: &Self::Output) {
        let globals = self.globals();
        if globals.quiet || globals.json {
            return;
        }

        let theme = self.ctx().theme();
        let rows: Vec<SpaceRow> = output.spaces.iter().map(SpaceRow::from).collect();

        let mut table = Table::new(&rows);
        table
            .with(Style::modern_rounded())
            .with(Modify::new(Rows::first()).with(theme.primary().to_tabled() | TabledColor::BOLD));

        for (i, space) in output.spaces.iter().enumerate() {
            if space.active {
                table.with(Modify::new(Cell::new(i + 1, 0)).with(theme.highlight().to_tabled()));
            }
        }

        let _ = writeln!(stdout(), "{table}");
    }
}

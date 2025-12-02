//! Initialize a new Flow graph.

use clap::Args;
use flow_core::config::Config;
use flow_core::graph::Graph;
use inquire::Text;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;
use std::path::PathBuf;

use crate::common::{path_to_display_string, Command, GlobalArgs};
use crate::error::CliError;

/// Output structure for the init command.
#[derive(Debug, Clone, Serialize)]
pub struct InitOutput {
    pub name: String,
    pub path: String,
}

/// Arguments for the init command.
#[derive(Args)]
pub struct InitArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Directory path for new graph (interactive mode if not provided)
    pub path: Option<PathBuf>,

    /// Graph name (defaults to directory name)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Initialize with template structure
    #[arg(short, long)]
    pub template: Option<String>,
}

/// Init command implementation.
pub struct InitCommand {
    args: InitArgs,
}

impl Command for InitCommand {
    type Args = InitArgs;
    type Output = InitOutput;

    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }

    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }

    fn interactive(&mut self) -> Result<()> {
        // Validate path is provided when not in interactive mode
        if self.args.path.is_none() {
            self.args.global.info("Entering interactive mode");

            // Ask for path
            let path_input = Text::new("Directory path:")
                .with_default(".")
                .with_help_message("Path where the graph will be initialized")
                .prompt()
                .map_err(CliError::from)?;

            self.args.path = Some(PathBuf::from(path_input));

            // Ask for name if not already provided
            if self.args.name.is_none() {
                let name_input = Text::new("Graph name:")
                    .with_help_message("Leave empty to use directory name")
                    .prompt()
                    .map_err(CliError::from)?;

                if !name_input.trim().is_empty() {
                    self.args.name = Some(name_input);
                }
            }
        }

        Ok(())
    }

    fn run(self) -> Result<Self::Output> {
        // Validate path is provided
        let path = self
            .args
            .path
            .ok_or_else(|| CliError::missing_argument("path"))?;
        let name = self.args.name;

        let mut config = Config::load()?;

        // Check if path already exists and has a .flow directory
        if Graph::exists(path.as_path()) {
            return Err(CliError::graph_already_exists(path).into());
        }

        self.args
            .global
            .step(&format!("Initializing graph at {}", path.display()));

        let graph = Graph::init(&path, name.as_ref())?;

        // TODO: Handle template parameter when template support is implemented
        if self.args.template.is_some() {
            self.args
                .global
                .warning("Template support not yet implemented");
        }

        self.args.global.step("Registering graph in configuration");
        config.add_graph(&graph)?;

        let canonical_path = path.canonicalize().into_diagnostic()?;
        let display_path = path_to_display_string(&canonical_path);

        Ok(InitOutput {
            name: graph.name().to_string(),
            path: display_path,
        })
    }

    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.success("Graph initialized successfully");
        global.blank();
        global.kv("Name", &output.name);
        global.kv("Path", &output.path);
    }
}

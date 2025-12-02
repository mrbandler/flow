//! Open an existing Flow graph.

use clap::Args;
use flow_core::config::Config;
use flow_core::graph::Graph;
use inquire::Select;
use miette::{IntoDiagnostic, Result};
use serde::Serialize;
use std::path::PathBuf;

use crate::common::{path_to_display_string, Command, GlobalArgs};
use crate::error::CliError;

/// Output structure for the open command.
#[derive(Debug, Clone, Serialize)]
pub struct OpenOutput {
    pub name: String,
    pub path: String,
}

/// Arguments for the open command.
#[derive(Args)]
pub struct OpenArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Path to existing graph or registered graph name (enters interactive mode if not provided)
    pub path_or_name: Option<String>,

    /// Make this the default graph
    #[arg(long)]
    pub set_default: bool,
}

/// Open command implementation.
pub struct OpenCommand {
    args: OpenArgs,
}

impl Command for OpenCommand {
    type Args = OpenArgs;
    type Output = OpenOutput;

    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }

    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }

    fn interactive(&mut self) -> Result<()> {
        // Only enter interactive mode if path_or_name is not provided
        if self.args.path_or_name.is_none() {
            self.args.global.info("Entering interactive mode");

            let config = Config::load()?;
            let all_graphs = config.all_graphs();

            if all_graphs.is_empty() {
                return Err(CliError::Other {
                    message: "No registered graphs found. Use 'flow init' to create a graph."
                        .to_string(),
                }
                .into());
            }

            let active_graph_name = config.get_active_space_name();

            // Create display options with name and path, marking the active graph
            let options: Vec<String> = all_graphs
                .iter()
                .map(|(name, graph_config)| {
                    let display_path = path_to_display_string(&graph_config.path);
                    let is_active = active_graph_name == Some(name.as_str());
                    if is_active {
                        format!("{} ({}) [active]", name, display_path)
                    } else {
                        format!("{} ({})", name, display_path)
                    }
                })
                .collect();

            let selection = Select::new("Select a graph to open:", options)
                .prompt()
                .map_err(CliError::from)?;

            // Extract the graph name from the selection (before the path in parentheses)
            let name = selection
                .split(" (")
                .next()
                .unwrap_or(&selection)
                .to_string();

            self.args.path_or_name = Some(name);
        }

        Ok(())
    }

    fn run(self) -> Result<Self::Output> {
        // Validate path_or_name is provided
        let path_or_name = self
            .args
            .path_or_name
            .ok_or_else(|| CliError::missing_argument("path_or_name"))?;

        self.args
            .global
            .step(&format!("Looking for graph: {}", path_or_name));

        let mut config = Config::load()?;

        // Try to interpret as a registered graph name or path first
        let graph = if let Some(graph_config) = config.get_space_config(&path_or_name) {
            // It's a registered graph (by name or path)
            self.args.global.debug(
                "Found registered graph at",
                &graph_config.path.display().to_string(),
            );

            let graph = Graph::load(&graph_config.path)?;
            config.set_active_space(&path_or_name)?;
            graph
        } else {
            // Try to interpret as a path
            let path = PathBuf::from(&path_or_name);

            // Check if the path exists
            if !path.exists() {
                return Err(CliError::graph_not_found(&path_or_name).into());
            }

            self.args
                .global
                .step(&format!("Loading graph from path: {}", path.display()));

            // Try to load the graph to validate it
            let graph = Graph::load(&path).map_err(|_| CliError::invalid_graph(path.clone()))?;

            // Canonicalize path before checking if registered (config stores canonical paths)
            let canonical_check_path = path.canonicalize().into_diagnostic()?;
            let is_registered = config.is_space_registered(&canonical_check_path);

            if is_registered {
                // If already registered, set it as active
                self.args
                    .global
                    .debug("Status", "Graph already registered, setting as active");
                config.set_active_space(&canonical_check_path.to_string_lossy())?;
            } else {
                // If not registered, add it to the config
                self.args
                    .global
                    .step("Registering new graph in configuration");
                config.add_graph(&graph)?;
            }

            graph
        };

        let canonical_path = graph.path().canonicalize().into_diagnostic()?;
        let display_path = path_to_display_string(&canonical_path);

        Ok(OpenOutput {
            name: graph.name().to_string(),
            path: display_path,
        })
    }

    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.success("Graph opened successfully");
        global.blank();
        global.kv("Name", &output.name);
        global.kv("Path", &output.path);
    }
}

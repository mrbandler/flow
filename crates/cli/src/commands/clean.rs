//! Remove orphaned graphs from configuration.

use clap::Args;
use flow_core::config::Config;
use flow_core::graph::Graph;
use miette::Result;
use serde::Serialize;

use crate::common::{path_to_display_string, Command, GlobalArgs};

/// Output structure for a removed graph entry.
#[derive(Debug, Clone, Serialize)]
pub struct RemovedGraph {
    name: String,
    path: String,
    reason: String,
}

/// Output structure for a kept graph entry.
#[derive(Debug, Clone, Serialize)]
pub struct KeptGraph {
    name: String,
    path: String,
}

/// Output structure for the clean command.
#[derive(Debug, Clone, Serialize)]
pub struct CleanOutput {
    checked: usize,
    removed: Vec<RemovedGraph>,
    kept: Vec<KeptGraph>,
    dry_run: bool,
}

/// Arguments for the clean command.
#[derive(Args)]
pub struct CleanArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Show what would be removed without making changes
    #[arg(long)]
    pub dry_run: bool,
}

/// Clean command implementation.
pub struct CleanCommand {
    args: CleanArgs,
}

impl Command for CleanCommand {
    type Args = CleanArgs;
    type Output = CleanOutput;

    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }

    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }

    fn run(self) -> Result<Self::Output> {
        self.args.global.step("Loading configuration");
        let mut config = Config::load()?;

        let graph_count = config.graph_count();
        self.args.global.info(&format!(
            "Checking {} registered graph{}",
            graph_count,
            if graph_count == 1 { "" } else { "s" }
        ));

        let mut removed = Vec::new();
        let mut kept = Vec::new();
        let graphs_to_check = config.all_graphs();

        for (name, graph_config) in graphs_to_check {
            let path = &graph_config.path;
            let display_path = path_to_display_string(path);

            self.args
                .global
                .debug("Checking", &format!("{} ({})", name, display_path));

            // Check if directory exists
            if !path.exists() {
                if self.args.dry_run {
                    self.args
                        .global
                        .warning(&format!("Would remove {} - directory not found", name));
                } else {
                    self.args
                        .global
                        .step(&format!("Removing {} - directory not found", name));
                }

                removed.push(RemovedGraph {
                    name: name.clone(),
                    path: display_path.clone(),
                    reason: "directory not found".to_string(),
                });

                if !self.args.dry_run {
                    config.unregister_space(&name)?;
                }
                continue;
            }

            // Check if it's a valid Flow graph
            if !Graph::exists(path) {
                if self.args.dry_run {
                    self.args
                        .global
                        .warning(&format!("Would remove {} - not a valid graph", name));
                } else {
                    self.args
                        .global
                        .step(&format!("Removing {} - not a valid graph", name));
                }

                removed.push(RemovedGraph {
                    name: name.clone(),
                    path: display_path.clone(),
                    reason: "not a valid graph".to_string(),
                });

                if !self.args.dry_run {
                    config.unregister_space(&name)?;
                }
                continue;
            }

            // Graph is valid, keep it
            self.args
                .global
                .debug("Keeping", &format!("{} - valid graph", name));

            kept.push(KeptGraph {
                name: name.clone(),
                path: display_path,
            });
        }

        Ok(CleanOutput {
            checked: graph_count,
            removed,
            kept,
            dry_run: self.args.dry_run,
        })
    }

    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.heading("Clean Results");
        global.blank();

        if !output.removed.is_empty() {
            if output.dry_run {
                global.warning("Graphs that would be removed:");
            } else {
                global.info("Removed graphs:");
            }
            for r in &output.removed {
                global.kv(&r.name, &format!("{} ({})", r.path, r.reason));
            }
            global.blank();
        }

        if !output.kept.is_empty() && global.verbose {
            global.info("Kept graphs:");
            for k in &output.kept {
                global.kv(&k.name, &k.path);
            }
            global.blank();
        }

        if output.dry_run {
            global.warning(&format!(
                "Dry run: {} graph{} would be removed",
                output.removed.len(),
                if output.removed.len() == 1 { "" } else { "s" }
            ));
        } else if output.removed.is_empty() {
            global.success("No orphaned graphs found");
        } else {
            global.success(&format!(
                "Cleaned {} orphaned graph{} from configuration",
                output.removed.len(),
                if output.removed.len() == 1 { "" } else { "s" }
            ));
        }
    }
}

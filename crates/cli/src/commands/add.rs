//! Add a node to today's journal page.

use clap::Args;
use miette::Result;
use serde::Serialize;

use crate::common::{Command, GlobalArgs};

/// Output structure for the add command.
#[derive(Debug, Clone, Serialize)]
pub struct AddOutput {
    pub content: String,
    pub message: String,
}

/// Arguments for the add command.
#[derive(Args)]
pub struct AddArgs {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Content to add to today's journal
    pub content: String,
}

/// Add command implementation.
pub struct AddCommand {
    args: AddArgs,
}

impl Command for AddCommand {
    type Args = AddArgs;
    type Output = AddOutput;

    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }

    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }

    fn run(self) -> Result<Self::Output> {
        self.args
            .global
            .print_verbose(&format!("Adding content: {}", self.args.content));

        // Load graph using global.load_graph() which respects --graph flag
        self.args.global.print_verbose("Loading graph");
        let mut graph = self.args.global.load_graph()?;

        graph.add(&self.args.content)?;

        Ok(AddOutput {
            content: self.args.content.clone(),
            message: "Added to today's journal".to_string(),
        })
    }

    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.print(&output.message);
    }
}

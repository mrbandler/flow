use clap::Subcommand;
use flow_core::Graph;
use std::path::PathBuf;

/// CLI commands.
///
/// # Variants
///
/// - `Init` - Initializes a graph.
#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,

        #[arg(short, long)]
        name: Option<String>,
    },

    Add {
        content: String,

        #[arg(short, long)]
        graph: PathBuf, // FIXME: Path to the graph for now.
    },
}

/// Runs the CLI.
///
/// # Arguments
///
/// - `cmd` (`Commands`) - CLI command to run.
///
/// # Returns
///
/// - `anyhow::Result<()>` - Nothing.
///
/// # Errors
///
/// Describe possible errors.
pub fn run(cmd: Commands) -> anyhow::Result<()> {
    match cmd {
        Commands::Init { path, name } => {
            Graph::init(&path, name.as_ref())?;
        }
        Commands::Add { content, graph } => {
            let mut g = Graph::load(&graph)?;
            g.add(&content)?;
        }
    }

    Ok(())
}

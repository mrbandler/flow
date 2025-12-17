//! Flow TUI Binary
//!
//! This binary includes the CLI and Terminal User Interface.
//! Build with: `cargo build --package flow --features tui --bin flow-tui`

use clap::{CommandFactory, Parser, Subcommand};
use console::set_colors_enabled;
use miette::{IntoDiagnostic, Result};

#[derive(Parser)]
#[command(name = "flow-tui")]
#[command(about = "Note taking for developers - with Terminal UI")]
#[command(version)]
struct FlowTui {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the Terminal User Interface
    Tui,
    #[command(flatten)]
    Cli(flow_cli::Commands),
}

fn main() -> Result<()> {
    // Force enable colors for console crate
    set_colors_enabled(true);

    // Set up miette for beautiful error reporting with fancy rendering
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .color(true)
                .context_lines(3)
                .tab_width(4)
                .force_graphical(true)
                .build(),
        )
    }))
    .expect("Failed to set miette hook");

    run()
}

fn run() -> Result<()> {
    let app = FlowTui::parse();

    match app.command {
        Some(Commands::Tui) | None => {
            // Default to TUI when no command is specified
            flow_tui::run()?;
        },
        Some(Commands::Cli(cmd)) => {
            flow_cli::run(cmd)?;
        },
    }

    Ok(())
}

//! Flow Desktop Binary
//!
//! This binary includes CLI commands plus the desktop GUI application.
//! Build with: `cargo build --package flow --features desktop --bin flow-desktop`

use clap::{CommandFactory, Parser, Subcommand};
use console::set_colors_enabled;
use miette::{IntoDiagnostic, Result};

#[derive(Parser)]
#[command(name = "flow-desktop")]
#[command(about = "Note taking for developers - Desktop edition")]
#[command(version)]
struct Flow {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the desktop GUI application
    Desktop,
    /// CLI commands
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
    let flow = Flow::parse();

    match flow.command {
        Some(Commands::Desktop) => flow_desktop::run(),

        Some(Commands::Cli(cmd)) => flow_cli::run(cmd),

        // Default to launching the desktop app when no command is provided
        None => {
            // If run without arguments, launch the desktop GUI
            if std::env::args().len() == 1 {
                flow_desktop::run()
            } else {
                Flow::command().print_help().into_diagnostic()?;
                std::process::exit(1);
            }
        },
    }
}

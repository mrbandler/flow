//! Flow - Note taking for developers.
//!
//! This is the main entry point for the Flow application. It provides a unified
//! binary that can operate in multiple modes:
//!
//! - **CLI** (default) - Command-line interface for space and note management
//! - **TUI** - Terminal user interface (with `tui` feature)
//! - **GUI** - Desktop graphical interface (with `gui` feature)
//! - **Server** - HTTP server mode (with `server` feature)

use clap::{CommandFactory, Parser, Subcommand};
use miette::{IntoDiagnostic, Result};

/// Builds the version string, appending enabled feature flags.
fn version() -> &'static str {
    let version = env!("CARGO_PKG_VERSION");
    let features: &[&str] = &[
        #[cfg(feature = "tui")]
        "tui",
        #[cfg(feature = "gui")]
        "gui",
        #[cfg(feature = "server")]
        "server",
    ];

    if features.is_empty() {
        // Leak the string to get a 'static lifetime (only called once)
        Box::leak(version.to_string().into_boxed_str())
    } else {
        let feature_str = features.join(", ");
        Box::leak(format!("{version} ({feature_str})").into_boxed_str())
    }
}

/// Top-level CLI argument parser.
#[derive(Parser)]
#[command(name = "flow")]
#[command(version = version())]
#[command(about = "Flow - Note taking for developers")]
struct Flow {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands for the Flow binary.
#[derive(Subcommand)]
enum Commands {
    /// Launch the terminal user interface.
    #[cfg(feature = "tui")]
    Tui,
    /// Launch the desktop graphical interface.
    #[cfg(feature = "gui")]
    Gui,
    /// Start the HTTP server.
    #[cfg(feature = "server")]
    Serve,
    /// CLI commands (init, etc.).
    #[command(flatten)]
    Cli(flow_cli::Commands),
}

/// Entry point for the fat binary.
#[tokio::main]
async fn main() -> Result<()> {
    miette::set_panic_hook();

    let flow = Flow::parse();
    match flow.command {
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => flow_tui::run().await?,
        #[cfg(feature = "gui")]
        Some(Commands::Gui) => flow_gui::run().await?,
        #[cfg(feature = "server")]
        Some(Commands::Serve) => flow_server::run().await?,
        Some(Commands::Cli(cmd)) => flow_cli::run(cmd).await?,
        None => {
            // TODO: Maybe later we can just launch the UI the user selected in the configuration.

            Flow::command().print_help().into_diagnostic()?;
            std::process::exit(1);
        },
    }

    Ok(())
}

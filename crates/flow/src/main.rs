use clap::{CommandFactory, Parser, Subcommand};
use miette::{IntoDiagnostic, Result};

/// Build the version string with compiled features.
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

/// Commands for the fat binary.
#[derive(Parser)]
#[command(name = "flow")]
#[command(version = version())]
#[command(about = "Flow - Note taking for developers")]
struct Flow {
    /// Space to use (registered name or path), falls back to registered default space if not set.
    #[arg(short, long, global = true)]
    space: Option<String>,

    /// Sub-commands.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Sub-commands.
#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "tui")]
    Tui,
    #[cfg(feature = "gui")]
    Gui,
    #[cfg(feature = "server")]
    Serve,
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
        Some(Commands::Cli(cmd)) => flow_cli::run(&cmd)?,
        None => {
            // TODO: Maybe later we can just launch the UI the user selected in the configuration.

            Flow::command().print_help().into_diagnostic()?;
            std::process::exit(1);
        },
    }

    Ok(())
}

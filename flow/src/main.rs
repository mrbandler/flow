use clap::{CommandFactory, Parser, Subcommand};
use console::set_colors_enabled;
use miette::{IntoDiagnostic, Result};

#[derive(Parser)]
#[command(name = "flow")]
#[command(about = "Note taking for developers")]
struct Flow {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "tui")]
    Tui,
    #[cfg(feature = "desktop")]
    Desktop,
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
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => flow_tui::run()?,

        #[cfg(feature = "desktop")]
        Some(Commands::Desktop) => flow_desktop::run()?,

        Some(Commands::Cli(cmd)) => flow_cli::run(cmd)?,

        None => {
            Flow::command().print_help().into_diagnostic()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

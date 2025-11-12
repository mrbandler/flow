use clap::{CommandFactory, Parser, Subcommand};

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

fn main() -> anyhow::Result<()> {
    let flow = Flow::parse();

    match flow.command {
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => flow_tui::run()?,

        #[cfg(feature = "desktop")]
        Some(Commands::Desktop) => flow_desktop::run()?,

        Some(Commands::Cli(cmd)) => flow_cli::run(cmd)?,

        None => {
            Flow::command().print_help()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

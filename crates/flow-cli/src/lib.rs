use clap::Subcommand;
use miette::Result;

#[derive(Subcommand)]
pub enum Commands {
    Test,
}

/// Run the CLI with the given command.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::Test => println!("This is a test"),
    }

    Ok(())
}

use clap::Subcommand;
use flow_core::Space;
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
pub async fn run(cmd: &Commands) -> Result<()> {
    let _space = Space::load("test".to_owned()).await?;

    match cmd {
        Commands::Test => println!("This is a test"),
    }

    Ok(())
}

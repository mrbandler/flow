//! The `space` subcommand for managing Flow spaces.
//!
//! This module contains all space-related commands including initialization,
//! listing, and management of Flow spaces.

use clap::Subcommand;
use miette::Result;

pub mod init;
pub mod list;

/// Space management commands.
///
/// These commands handle the lifecycle of Flow spaces, from creation
/// to configuration and removal.
#[derive(Subcommand)]
pub enum Space {
    /// Initialize a new space.
    Init(init::Arguments),

    /// List all registered spaces.
    List(list::Arguments),
}

impl Space {
    /// Run the space subcommand.
    ///
    /// # Errors
    ///
    /// Returns an error if the command execution fails.
    pub async fn run(self) -> Result<()> {
        use crate::commands::Command;

        match self {
            Self::Init(args) => init::Init::new(args).run().await,
            Self::List(args) => list::List::new(args).run().await,
        }
    }
}

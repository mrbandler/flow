//! The `space` subcommand for managing Flow spaces.
//!
//! This module contains all space-related commands including initialization,
//! listing, and management of Flow spaces.

use core::fmt;
use std::path::PathBuf;

use clap::Subcommand;
use flow_common::PathExt;
use flow_core::{Config, Locator};
use miette::Result;

pub mod init;
pub mod list;
pub mod register;
pub mod switch;
pub mod unregister;

/// A space option for interactive selection.
///
/// Displays as "name (path)" or "name (path) (active)" for the currently active space.
pub struct SpaceOption {
    pub name: String,
    pub path: PathBuf,
    pub is_active: bool,
}

impl SpaceOption {
    /// Builds a list of space options from the config for interactive selection.
    ///
    /// Returns the options and the index of the default selection based on the provided locator.
    pub fn from_config(config: &Config, default_locator: Option<&Locator>) -> (Vec<Self>, usize) {
        let active = config.active().map(|s| s.name.as_str());
        let registered = config.spaces();

        let options: Vec<Self> = registered
            .iter()
            .map(|space| Self {
                name: space.name.clone(),
                path: space.path.clone(),
                is_active: active == Some(space.name.as_str()),
            })
            .collect();

        // Find the default index based on the locator
        let default_index = default_locator
            .and_then(|loc| match loc {
                Locator::Path(p) => registered.iter().position(|s| &s.path == p),
                Locator::Name(n) => registered.iter().position(|s| &s.name == n),
            })
            .unwrap_or(0);

        (options, default_index)
    }
}

impl fmt::Display for SpaceOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_active {
            write!(f, "{} ({}) ← active", self.name, self.path.normalize_to_string())
        } else {
            write!(f, "{} ({})", self.name, self.path.normalize_to_string())
        }
    }
}

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

    /// Switch the active space.
    Switch(switch::Arguments),

    /// Register an existing space.
    Register(register::Arguments),

    /// Unregister a space.
    Unregister(unregister::Arguments),
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
            Self::Switch(args) => switch::Switch::new(args).run().await,
            Self::Register(args) => register::Register::new(args).run().await,
            Self::Unregister(args) => unregister::Unregister::new(args).run().await,
        }
    }
}

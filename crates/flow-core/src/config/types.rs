//! Configuration data types.
//!
//! This module provides the data structures for Flow's configuration system,
//! including user settings and registered spaces.
//!
//! # Overview
//!
//! Flow stores its configuration in `~/.config/flow/` with two files:
//!
//! - `config.json` - User preferences and settings
//! - `spaces.json` - Registered spaces and the active space
//!
//! # Serialization
//!
//! All types are serialized to JSON using [`serde`]. The format is designed
//! to be human-readable and forward-compatible.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// User configuration settings.
///
/// This struct holds user preferences that affect Flow's behavior.
/// It is stored in `~/.config/flow/config.json`.
///
/// # Serialization Format
///
/// ```json
/// {
///     "version": "0.1.0"
/// }
/// ```
///
/// # Future Extensions
///
/// This struct is intentionally minimal. Future versions may add fields for:
/// - Editor preferences
/// - Default space settings
/// - UI customization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    /// The Flow version that created this configuration.
    ///
    /// Used for future migrations if the configuration format changes.
    pub version: String,
}

/// A registered space entry.
///
/// Each registered space has a name and a filesystem path. The name
/// is used for quick access via commands like `flow open <name>`.
///
/// # Serialization Format
///
/// ```json
/// {
///     "name": "personal",
///     "path": "/home/user/spaces/personal"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredSpace {
    /// The human-readable name of the space.
    ///
    /// This name is unique within the registry and is used to
    /// identify the space in commands.
    pub name: String,

    /// The filesystem path to the space directory.
    ///
    /// This is an absolute path to the directory containing the
    /// `.flow/` subdirectory.
    pub path: PathBuf,
}

/// Registry of known spaces and the active space.
///
/// This struct maintains the list of all registered spaces and tracks
/// which space is currently active. It is stored in `~/.config/flow/spaces.json`.
///
/// # Serialization Format
///
/// ```json
/// {
///     "active": "personal",
///     "spaces": [
///         {
///             "name": "personal",
///             "path": "/home/user/spaces/personal"
///         },
///         {
///             "name": "work",
///             "path": "/home/user/spaces/work"
///         }
///     ]
/// }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Spaces {
    /// The name of the currently active space.
    ///
    /// This is `None` if no space has been activated yet.
    /// When set, it must match the name of a space in the `spaces` list.
    pub active: Option<String>,

    /// The list of registered spaces.
    ///
    /// Each space has a unique name that can be used to quickly
    /// open or reference it.
    pub spaces: Vec<RegisteredSpace>,
}

//! Space locator types.
//!
//! This module provides [`Locator`], a flexible way to identify spaces
//! either by name or by filesystem path.
//!
//! # Overview
//!
//! When working with Flow spaces, you often need to specify which space
//! to operate on. The [`Locator`] enum provides two ways to do this:
//!
//! - **By name**: Use a human-readable name that is resolved from the
//!   global Flow configuration (e.g., `"personal"`, `"work"`).
//! - **By path**: Use an explicit filesystem path to the space directory.
//!
//! # Type Conversions
//!
//! `Locator` implements [`From`] for common string and path types, making
//! it easy to use with APIs that accept `impl Into<Locator>`:
//!
//! - `String` and `&str` are interpreted as space names.
//! - `PathBuf` and `&Path` are interpreted as filesystem paths.

use std::fmt;
use std::path::{Path, PathBuf};

/// A way to locate a [`Space`](super::Space).
///
/// Spaces can be identified either by their human-readable name
/// (which is resolved from a configuration directory) or by
/// their explicit filesystem path.
///
/// # Examples
///
/// ```
/// use flow_core::Locator;
/// use std::path::PathBuf;
///
/// // From a string (interpreted as name)
/// let loc: Locator = "my-notes".into();
///
/// // From a PathBuf (interpreted as path)
/// let loc: Locator = PathBuf::from("/home/user/notes").into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Locator {
    /// Locate a space by its registered name.
    ///
    /// The name is resolved from the global Flow configuration file,
    /// which maps names to filesystem paths. This is the preferred
    /// way to reference spaces in most user-facing contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_core::Locator;
    ///
    /// let locator = Locator::Name("personal".to_string());
    /// ```
    Name(String),

    /// Locate a space by its explicit filesystem path.
    ///
    /// This bypasses the global configuration and directly references
    /// a space directory. Useful for working with spaces that haven't
    /// been registered, or for tools that operate on arbitrary paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use flow_core::Locator;
    ///
    /// let locator = Locator::Path(PathBuf::from("/home/user/notes"));
    /// ```
    Path(PathBuf),
}

impl From<String> for Locator {
    fn from(s: String) -> Self {
        Self::Name(s)
    }
}

impl From<&String> for Locator {
    fn from(s: &String) -> Self {
        Self::Name(s.clone())
    }
}

impl From<&str> for Locator {
    fn from(s: &str) -> Self {
        Self::Name(s.to_owned())
    }
}

impl From<PathBuf> for Locator {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&Path> for Locator {
    fn from(path: &Path) -> Self {
        Self::Path(path.to_path_buf())
    }
}

impl fmt::Display for Locator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{name}"),
            Self::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

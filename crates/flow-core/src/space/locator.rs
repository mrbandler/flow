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
//! # Parsing
//!
//! `Locator` implements [`FromStr`] which intelligently determines whether
//! the input is a name or a path based on its structure:
//!
//! - Single-component strings like `"my-notes"` are interpreted as names.
//! - Strings with path separators, or starting with `.` or `..`, are paths.
//! - Absolute paths (starting with `/` or drive letters) are paths.
//!
//! # Type Conversions
//!
//! `Locator` also implements [`From`] for explicit conversions:
//!
//! - `String` and `&str` are always interpreted as space names.
//! - `PathBuf` and `&Path` are always interpreted as filesystem paths.

use std::fmt;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;

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
    /// Converts a string to a `Locator`, using the same detection logic as `FromStr`.
    ///
    /// This allows clap argument parsing to correctly detect paths vs names.
    fn from(s: String) -> Self {
        s.parse().expect("Locator::from_str is infallible")
    }
}

impl From<&String> for Locator {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for Locator {
    fn from(s: &str) -> Self {
        s.parse().expect("Locator::from_str is infallible")
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

impl FromStr for Locator {
    type Err = std::convert::Infallible;

    /// Parses a string into a `Locator`, automatically detecting whether
    /// it represents a name or a path.
    ///
    /// # Detection Rules
    ///
    /// The input is treated as a path if any of the following are true:
    /// - It starts with a root directory (`/` on Unix, `\` on Windows)
    /// - It starts with a Windows drive prefix (`C:\`, etc.)
    /// - It starts with `.` (current directory) or `..` (parent directory)
    /// - It contains multiple path components (e.g., `foo/bar`)
    ///
    /// Otherwise, it is treated as a space name.
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_core::Locator;
    ///
    /// // Single component -> Name
    /// let loc: Locator = "my-notes".parse().unwrap();
    /// assert!(matches!(loc, Locator::Name(_)));
    ///
    /// // Relative path with ./ -> Path
    /// let loc: Locator = "./projects".parse().unwrap();
    /// assert!(matches!(loc, Locator::Path(_)));
    ///
    /// // Multiple components -> Path
    /// let loc: Locator = "foo/bar".parse().unwrap();
    /// assert!(matches!(loc, Locator::Path(_)));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = Path::new(s);
        let mut components = path.components();

        let is_path = match components.next() {
            // Starts with /, \, C:\, ., or ..
            Some(Component::RootDir | Component::Prefix(_) | Component::CurDir | Component::ParentDir) => true,
            // Has multiple components (e.g., "foo/bar")
            Some(Component::Normal(_)) => components.next().is_some(),
            None => false,
        };

        if is_path {
            Ok(Self::Path(PathBuf::from(s)))
        } else {
            Ok(Self::Name(s.to_owned()))
        }
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

//! Space management for Flow.
//!
//! A [`Space`] is the primary organizational unit in Flow — it represents
//! a workspace containing notes, configuration, and metadata. Think of it
//! as a "vault" or "notebook" that groups related content together.
//!
//! # Overview
//!
//! Spaces can be:
//! - **Initialized** at a new location with [`Space::init`]
//! - **Loaded** from an existing location with [`Space::load`]
//!
//! Each space has a human-readable name and lives at a specific filesystem
//! path. Spaces can be located either by name (resolved from global
//! configuration) or by explicit path using a [`Locator`].
//!
//! # Examples
//!
//! ## Creating a new space
//!
//! ```no_run
//! use std::path::Path;
//! use flow_core::Space;
//!
//! # async fn example() -> miette::Result<()> {
//! // Initialize a new space called "personal" at the given path
//! let space = Space::init(Path::new("./my-notes"), "personal").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Loading an existing space
//!
//! ```no_run
//! use flow_core::Space;
//!
//! # async fn example() -> miette::Result<()> {
//! // Load by name (resolved from global configuration)
//! let space = Space::load("personal").await?;
//!
//! // Or load by explicit path
//! let space = Space::load(std::path::PathBuf::from("./my-notes")).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Architecture
//!
//! The public [`Space`] struct is a thin wrapper around an internal
//! implementation (`DefaultSpace`). This design
//! allows us to:
//!
//! - Keep the public API simple and stable
//! - Inject different filesystem implementations for testing
//! - Potentially support different space backends in the future

use std::path::Path;

use miette::Result;

use crate::filesystem::LocalFilesystem;
use crate::space::default::DefaultSpace;
use crate::space::traits::Space as SpaceTrait;

mod default;
mod locator;
mod metadata;
mod traits;

pub use self::locator::Locator;
pub use self::metadata::Metadata;

/// A Flow workspace containing notes, configuration, and metadata.
///
/// `Space` is the main entry point for interacting with a Flow workspace.
/// It provides methods to initialize new spaces and load existing ones.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use flow_core::Space;
///
/// # async fn example() -> miette::Result<()> {
/// // Create a new space
/// let space = Space::init(Path::new("./notes"), "work").await?;
///
/// // Later, load it by name
/// let space = Space::load("work").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Thread Safety
///
/// `Space` is `Send` and `Sync`, making it safe to share across threads
/// and use in async contexts.
pub struct Space {
    /// The underlying space implementation.
    #[allow(dead_code)]
    inner: DefaultSpace<LocalFilesystem>,
}

impl Space {
    /// Initialize a new space at the given path.
    ///
    /// This creates the necessary directory structure and configuration
    /// files for a new Flow space. The space will be registered with the
    /// given name, allowing it to be loaded by name in the future.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path where the space will be created.
    ///   The directory will be created if it doesn't exist.
    /// * `name` - A human-readable name for the space. This name can be
    ///   used later with [`Space::load`] to open the space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A space already exists at the given path
    /// - A space with the given name is already registered
    /// - The directory cannot be created (e.g., permission denied)
    /// - The configuration file cannot be written
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use flow_core::Space;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let space = Space::init(Path::new("./my-notes"), "personal").await?;
    /// println!("Space created successfully!");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use = "Space was initialized but never used"]
    pub async fn init(path: impl AsRef<Path> + Send + Sync, name: impl Into<String>) -> Result<Self> {
        let fs = LocalFilesystem;
        let inner = DefaultSpace::init(fs, path, name).await?;

        Ok(Self { inner })
    }

    /// Load an existing space.
    ///
    /// Spaces can be loaded either by their registered name or by an
    /// explicit filesystem path. The `locator` parameter accepts anything
    /// that can be converted into a [`Locator`], including:
    ///
    /// - `&str` or `String` — interpreted as a space name
    /// - `&Path` or `PathBuf` — interpreted as a filesystem path
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies which space to load. See [`Locator`] for
    ///   the different ways to specify a space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The space cannot be found (name not registered or path doesn't exist)
    /// - The space configuration is missing or corrupted
    /// - The filesystem cannot be read (e.g., permission denied)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Space;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// // Load by name
    /// let space = Space::load("personal").await?;
    ///
    /// // Load by path
    /// let space = Space::load(PathBuf::from("./my-notes")).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use = "Space was loaded but never used"]
    pub async fn load(locator: impl Into<Locator>) -> Result<Self> {
        let fs = LocalFilesystem;
        let inner = DefaultSpace::load(fs, locator.into()).await?;

        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add integration tests for Space
    //
    // Tests should cover:
    // - Initializing a new space
    // - Loading an existing space by name
    // - Loading an existing space by path
    // - Error cases (space not found, already exists, etc.)
}

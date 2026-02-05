//! Configuration trait definition.
//!
//! This module defines the [`Config`] trait, which provides the interface
//! for managing Flow's configuration, including registered spaces and
//! user settings.

use miette::Result;

use crate::filesystem::Filesystem;
use crate::space::Locator;

use super::types::{RegisteredSpace, Settings};
use crate::space::Space;

/// Configuration management for Flow.
///
/// This trait defines the interface for loading, saving, and managing
/// Flow's configuration. It handles both user settings and the registry
/// of known spaces.
///
/// # Implementors
///
/// - [`DefaultConfig`](super::default::DefaultConfig) - The standard implementation
///   using the filesystem abstraction.
///
/// # Examples
///
/// ```ignore
/// use flow_core::config::{Config, DefaultConfig};
/// use flow_core::filesystem::LocalFilesystem;
/// use flow_core::{Locator, Space};
///
/// let fs = LocalFilesystem;
/// let mut config = DefaultConfig::load(fs).await?;
///
/// // Initialize and register a space
/// let space = Space::init("./notes", "personal").await?;
/// config.register(&space).await?;
///
/// // Set it as active
/// config.set_active(&"personal".into()).await?;
/// ```
#[allow(dead_code)]
pub trait Config: Sized + Send + Sync {
    /// The filesystem implementation used for persistence.
    type Fs: Filesystem;

    /// Loads the configuration from disk.
    ///
    /// If the configuration files don't exist, they will be created with
    /// default values.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The configuration directory cannot be created
    /// - The configuration files cannot be read or written
    /// - The configuration files contain invalid JSON
    async fn load(fs: Self::Fs) -> Result<Self>;

    /// Registers a space in the configuration.
    ///
    /// The space will be added to the list of known spaces, allowing it
    /// to be opened by name in the future.
    ///
    /// # Arguments
    ///
    /// * `space` - A reference to the space to register.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A space with the same name is already registered
    /// - The configuration cannot be saved
    async fn register(&mut self, space: &Space) -> Result<()>;

    /// Unregisters a space from the configuration.
    ///
    /// The space will be removed from the list of known spaces. If the
    /// space was the active space, the active space will be cleared.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to unregister, either by name or path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The space is not registered
    /// - The configuration cannot be saved
    async fn unregister(&mut self, locator: &Locator, delete: bool) -> Result<()>;

    /// Checks if a space is registered in the configuration.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to check, either by name or path.
    ///
    /// # Returns
    ///
    /// `true` if the space is registered, `false` otherwise.
    #[must_use]
    async fn is_registered(&self, locator: &Locator) -> bool {
        self.find(locator).await.is_some()
    }

    /// Checks if the given space is the active space.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to check, either by name or path.
    ///
    /// # Returns
    ///
    /// `true` if the space is the currently active space, `false` otherwise.
    #[must_use]
    async fn is_active(&self, locator: &Locator) -> bool;

    /// Sets the active space.
    ///
    /// The active space is the default space used when no space is
    /// explicitly specified in commands.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to set as active, either by name or path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The space is not registered
    /// - The configuration cannot be saved
    async fn set_active(&mut self, locator: &Locator) -> Result<()>;

    /// Clears the active space.
    ///
    /// After calling this, no space will be active until one is explicitly
    /// set again.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be saved.
    async fn clear_active(&mut self) -> Result<()>;

    /// Returns the currently active space, if any.
    ///
    /// # Returns
    ///
    /// A reference to the active space, or `None` if no space is active.
    #[must_use]
    fn active(&self) -> Option<&RegisteredSpace>;

    /// Finds a registered space by name or path.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to find, either by name or path.
    ///
    /// # Returns
    ///
    /// A reference to the registered space, or `None` if not found.
    #[must_use]
    async fn find(&self, locator: &Locator) -> Option<&RegisteredSpace>;

    /// Returns all registered spaces.
    ///
    /// # Returns
    ///
    /// A slice containing all registered spaces.
    #[must_use]
    fn spaces(&self) -> &[RegisteredSpace];

    /// Returns the current user settings.
    ///
    /// # Returns
    ///
    /// A reference to the settings.
    #[must_use]
    fn settings(&self) -> &Settings;
}

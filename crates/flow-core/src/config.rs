//! Configuration management for Flow.
//!
//! This module provides the configuration system for Flow, managing both
//! user settings and the registry of known spaces.
//!
//! # Overview
//!
//! Flow stores its configuration in `~/.config/flow/` with two files:
//!
//! - `config.json` - User preferences and settings
//! - `spaces.json` - Registered spaces and the active space
//!
//! The [`Config`] struct provides the main interface for working with
//! configuration. It supports:
//!
//! - Registering and unregistering spaces
//! - Setting and clearing the active space
//! - Looking up spaces by name or path
//! - Accessing user settings
//!
//! # Examples
//!
//! ## Loading configuration
//!
//! ```no_run
//! use flow_core::Config;
//!
//! # async fn example() -> miette::Result<()> {
//! let config = Config::load().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Registering a space
//!
//! ```no_run
//! use std::path::Path;
//! use flow_core::{Config, Space};
//!
//! # async fn example() -> miette::Result<()> {
//! let space = Space::init(Path::new("./notes"), "personal").await?;
//! let mut config = Config::load().await?;
//! config.register(&space).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Setting the active space
//!
//! ```no_run
//! use flow_core::{Config, Locator};
//!
//! # async fn example() -> miette::Result<()> {
//! let mut config = Config::load().await?;
//! config.set_active(&"personal".into()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Architecture
//!
//! The public [`Config`] struct is a thin wrapper around an internal
//! implementation (`DefaultConfig`). This design allows us to:
//!
//! - Keep the public API simple and stable
//! - Inject different filesystem implementations for testing
//! - Potentially support different configuration backends in the future

use miette::Result;

use crate::filesystem::LocalFilesystem;
use crate::space::{Locator, Space};

use self::default::DefaultConfig;
use self::traits::Config as _;

mod default;
mod traits;
mod types;

pub use self::types::{RegisteredSpace, Settings};

/// Flow configuration manager.
///
/// `Config` is the main entry point for managing Flow's configuration.
/// It provides methods to register spaces, set the active space, and
/// access user settings.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use flow_core::{Config, Locator, Space};
///
/// # async fn example() -> miette::Result<()> {
/// // Load configuration (creates default if none exists)
/// let mut config = Config::load().await?;
///
/// // Initialize and register a space
/// let space = Space::init(Path::new("./work-notes"), "work").await?;
/// config.register(&space).await?;
///
/// // Set it as active
/// config.set_active(&"work".into()).await?;
///
/// // Look up a space
/// if let Some(registered) = config.find(&"work".into()).await {
///     println!("Found space at: {}", registered.path.display());
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Thread Safety
///
/// `Config` is `Send` and `Sync`, making it safe to share across threads
/// and use in async contexts.
pub struct Config {
    /// The underlying configuration implementation.
    inner: DefaultConfig<LocalFilesystem>,
}

impl Config {
    /// Loads the configuration from disk.
    ///
    /// If the configuration files don't exist, they will be created with
    /// default values in `~/.config/flow/`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The configuration directory cannot be created
    /// - The configuration files cannot be read or written
    /// - The configuration files contain invalid JSON
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Config;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let config = Config::load().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load() -> Result<Self> {
        let fs = LocalFilesystem;
        let inner = DefaultConfig::load(fs).await?;
        Ok(Self { inner })
    }

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
    /// - A space at the same path is already registered
    /// - The configuration cannot be saved
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use flow_core::{Config, Space};
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let space = Space::init(Path::new("./notes"), "personal").await?;
    /// let mut config = Config::load().await?;
    /// config.register(&space).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register(&mut self, space: &Space) -> Result<()> {
        self.inner.register(space).await
    }

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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::{Config, Locator};
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let mut config = Config::load().await?;
    /// config.unregister(&"personal".into(), false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unregister(&mut self, locator: &Locator, delete: bool) -> Result<()> {
        self.inner.unregister(locator, delete).await
    }

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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::{Config, Locator};
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let mut config = Config::load().await?;
    /// config.set_active(&"personal".into()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_active(&mut self, locator: &Locator) -> Result<()> {
        self.inner.set_active(locator).await
    }

    /// Clears the active space.
    ///
    /// After calling this, no space will be active until one is explicitly
    /// set again.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be saved.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Config;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let mut config = Config::load().await?;
    /// config.clear_active().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_active(&mut self) -> Result<()> {
        self.inner.clear_active().await
    }

    /// Returns the currently active space, if any.
    ///
    /// # Returns
    ///
    /// A reference to the active space, or `None` if no space is active.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Config;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let config = Config::load().await?;
    /// if let Some(space) = config.active() {
    ///     println!("Active space: {}", space.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn active(&self) -> Option<&RegisteredSpace> {
        self.inner.active()
    }

    /// Finds a registered space by name or path.
    ///
    /// # Arguments
    ///
    /// * `locator` - Identifies the space to find, either by name or path.
    ///
    /// # Returns
    ///
    /// A reference to the registered space, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::{Config, Locator};
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let config = Config::load().await?;
    /// if let Some(space) = config.find(&"personal".into()).await {
    ///     println!("Found at: {}", space.path.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find(&self, locator: &Locator) -> Option<&RegisteredSpace> {
        self.inner.find(locator).await
    }

    /// Returns all registered spaces.
    ///
    /// # Returns
    ///
    /// A slice containing all registered spaces.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Config;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let config = Config::load().await?;
    /// for space in config.spaces() {
    ///     println!("{}: {}", space.name, space.path.display());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn spaces(&self) -> &[RegisteredSpace] {
        self.inner.spaces()
    }

    /// Returns the current user settings.
    ///
    /// # Returns
    ///
    /// A reference to the settings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_core::Config;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let config = Config::load().await?;
    /// println!("Config version: {}", config.settings().version);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn settings(&self) -> &Settings {
        self.inner.settings()
    }
}

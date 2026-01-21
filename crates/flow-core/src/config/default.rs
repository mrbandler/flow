//! Default implementation of the [`Config`] trait.
//!
//! This module provides [`DefaultConfig`], the standard implementation of
//! the config trait used in production. It is generic over the filesystem
//! implementation to allow for dependency injection during testing.
//!
//! # Overview
//!
//! The [`DefaultConfig`] struct manages Flow's configuration, including:
//!
//! - User settings stored in `~/.config/flow/config.json`
//! - Registered spaces stored in `~/.config/flow/spaces.json`
//!
//! # Directory Structure
//!
//! Configuration is stored in the XDG config directory:
//!
//! ```text
//! ~/.config/flow/
//! ├── config.json    # User settings
//! └── spaces.json    # Registered spaces
//! ```

use std::path::PathBuf;

use cross_xdg::BaseDirs;
use miette::{IntoDiagnostic, Result};

use crate::errors::Error;
use crate::filesystem::Filesystem;
use crate::space::Locator;

use super::traits::Config;
use super::types::{RegisteredSpace, Settings, Spaces};
use crate::space::Space;

/// The directory name for Flow configuration under the XDG config home.
const CONFIG_DIR: &str = "flow";

/// The filename for user settings.
const SETTINGS_FILE: &str = "config.json";

/// The filename for registered spaces.
const SPACES_FILE: &str = "spaces.json";

/// The default implementation of Flow configuration.
///
/// `DefaultConfig` is generic over the filesystem implementation, allowing
/// different storage backends to be used. In production, this is typically
/// [`LocalFilesystem`](crate::filesystem::LocalFilesystem), while tests
/// can inject mock implementations.
///
/// # Type Parameters
///
/// * `F` - The filesystem implementation to use for all I/O operations.
///   Must implement [`Filesystem`] and be thread-safe (`Send + Sync`).
pub struct DefaultConfig<F: Filesystem> {
    /// The filesystem backend for all I/O operations.
    fs: F,

    /// The path to the configuration directory.
    config_dir: PathBuf,

    /// User settings.
    settings: Settings,

    /// Registered spaces.
    spaces: Spaces,
}

impl<F: Filesystem> DefaultConfig<F> {
    /// Returns the path to the configuration directory.
    ///
    /// Uses `cross-xdg` to determine the XDG config home, then appends
    /// the Flow-specific directory name.
    fn config_dir() -> Result<PathBuf> {
        let base_dirs = BaseDirs::new().into_diagnostic()?;
        Ok(base_dirs.config_home().join(CONFIG_DIR))
    }

    /// Persists the settings to disk.
    #[allow(dead_code)]
    async fn save_settings(&self) -> Result<()> {
        let path = self.config_dir.join(SETTINGS_FILE);
        let json = serde_json::to_string_pretty(&self.settings).into_diagnostic()?;
        self.fs.write(&path, json.as_bytes()).await
    }

    /// Persists the spaces to disk.
    async fn save_spaces(&self) -> Result<()> {
        let path = self.config_dir.join(SPACES_FILE);
        let json = serde_json::to_string_pretty(&self.spaces).into_diagnostic()?;
        self.fs.write(&path, json.as_bytes()).await
    }

    /// Finds a space by locator and returns its index.
    fn find_index(&self, locator: &Locator) -> Option<usize> {
        match locator {
            Locator::Name(name) => self.spaces.spaces.iter().position(|s| &s.name == name),
            Locator::Path(path) => self.spaces.spaces.iter().position(|s| &s.path == path),
        }
    }
}

impl<F: Filesystem> Config for DefaultConfig<F> {
    type Fs = F;

    async fn load(fs: Self::Fs) -> Result<Self>
    where
        Self: Sized,
    {
        let config_dir = Self::config_dir()?;

        if !fs.exists(&config_dir).await? {
            fs.create_dir_all(&config_dir).await?;
        }

        let settings_path = config_dir.join(SETTINGS_FILE);
        let settings = if fs.exists(&settings_path).await? {
            let json = fs.read_to_string(&settings_path).await?;
            serde_json::from_str(&json).into_diagnostic()?
        } else {
            let settings = Settings {
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            let json = serde_json::to_string_pretty(&settings).into_diagnostic()?;
            fs.write(&settings_path, json.as_bytes()).await?;
            settings
        };

        let spaces_path = config_dir.join(SPACES_FILE);
        let spaces = if fs.exists(&spaces_path).await? {
            let json = fs.read_to_string(&spaces_path).await?;
            serde_json::from_str(&json).into_diagnostic()?
        } else {
            let spaces = Spaces::default();
            let json = serde_json::to_string_pretty(&spaces).into_diagnostic()?;
            fs.write(&spaces_path, json.as_bytes()).await?;
            spaces
        };

        Ok(Self {
            fs,
            config_dir,
            settings,
            spaces,
        })
    }

    async fn register(&mut self, space: &Space) -> Result<()> {
        let name = space.name();
        let path = space.path();

        // Check if already registered by name
        if self.spaces.spaces.iter().any(|s| s.name == name) {
            return Err(Error::SpaceAlreadyRegistered(name.to_string()).into());
        }

        // Check if already registered by path
        if self.spaces.spaces.iter().any(|s| s.path == path) {
            return Err(Error::SpacePathAlreadyRegistered(path.to_path_buf()).into());
        }

        self.spaces.spaces.push(RegisteredSpace {
            name: name.to_string(),
            path: path.to_path_buf(),
        });

        self.save_spaces().await
    }

    async fn unregister(&mut self, locator: impl Into<Locator> + Send) -> Result<()> {
        let locator = locator.into();

        let index = self
            .find_index(&locator)
            .ok_or_else(|| Error::SpaceNotRegistered(locator.clone()))?;

        let removed = self.spaces.spaces.remove(index);

        // Clear active if it was the removed space
        if self.spaces.active.as_ref() == Some(&removed.name) {
            self.spaces.active = None;
        }

        self.save_spaces().await
    }

    async fn set_active(&mut self, locator: impl Into<Locator> + Send) -> Result<()> {
        let locator = locator.into();

        let space = self
            .find(locator.clone())
            .ok_or(Error::SpaceNotRegistered(locator))?;

        self.spaces.active = Some(space.name.clone());

        self.save_spaces().await
    }

    async fn clear_active(&mut self) -> Result<()> {
        self.spaces.active = None;
        self.save_spaces().await
    }

    fn active(&self) -> Option<&RegisteredSpace> {
        self.spaces
            .active
            .as_ref()
            .and_then(|name| self.spaces.spaces.iter().find(|s| &s.name == name))
    }

    fn find(&self, locator: impl Into<Locator>) -> Option<&RegisteredSpace> {
        let locator = locator.into();
        match locator {
            Locator::Name(name) => self.spaces.spaces.iter().find(|s| s.name == name),
            Locator::Path(path) => self.spaces.spaces.iter().find(|s| s.path == path),
        }
    }

    fn spaces(&self) -> &[RegisteredSpace] {
        &self.spaces.spaces
    }

    fn settings(&self) -> &Settings {
        &self.settings
    }
}

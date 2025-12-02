//! Flow Configuration Management
//!
//! This module provides configuration management using the confy crate
//! with XDG-style paths on all platforms (including Windows).
//!
//! On all platforms, the config file will be located at:
//! - `~/.config/flow/flow.toml`
//!
//! You can override the base directory with the `XDG_CONFIG_HOME` environment variable.

use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::space::Space;

const APP_NAME: &str = "flow";
const CONFIG_NAME: &str = "flow";

/// Main configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    spaces: HashMap<String, SpaceConfig>,
    #[serde(default)]
    active_space: Option<String>,
}

/// Space configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceConfig {
    pub path: PathBuf,
}

/// Default configuration.
impl Default for Config {
    fn default() -> Self {
        Self {
            spaces: HashMap::new(),
            active_space: None,
        }
    }
}

/// Configuration implementation.
impl Config {
    /// Loads the Flow configuration from disk
    ///
    /// This will use XDG-style paths on all platforms:
    /// - `~/.config/flow/flow.toml`
    ///
    /// If the config file doesn't exist, it will be created with default values.
    pub fn load() -> Result<Config> {
        confy::change_config_strategy(confy::ConfigStrategy::App);
        confy::load(APP_NAME, CONFIG_NAME)
            .into_diagnostic()
            .context("Failed to load Flow configuration")
    }

    /// Saves the Flow configuration to disk
    pub fn save(&self) -> Result<()> {
        confy::change_config_strategy(confy::ConfigStrategy::App);
        confy::store(APP_NAME, CONFIG_NAME, self)
            .into_diagnostic()
            .context("Failed to save Flow configuration")
    }

    /// Registers a space to the configuration
    ///
    /// # Arguments
    ///
    /// - `space` (`&Space`) - The space to add to the configuration
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if the space was added and saved successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration could not be saved to disk
    pub fn register_space(&mut self, space: &Space) -> Result<()> {
        // Canonicalize the path to ensure we always store absolute paths
        let canonical_path = space
            .path()
            .canonicalize()
            .into_diagnostic()
            .context(format!(
                "Failed to canonicalize path: {}",
                space.path().display()
            ))?;

        let entry = SpaceConfig {
            path: canonical_path,
        };

        let space_name = space.name().to_owned();
        self.spaces.insert(space_name.clone(), entry);

        // Set as active if it's the first space
        if self.spaces.len() == 1 {
            self.active_space = Some(space_name);
        }

        self.save()
    }

    /// Gets the active space configuration
    ///
    /// # Returns
    ///
    /// - `Option<&SpaceConfig>` - The active space configuration if one is set and exists
    pub fn get_active_space(&self) -> Option<&SpaceConfig> {
        self.active_space
            .as_ref()
            .and_then(|name| self.spaces.get(name))
    }

    /// Gets the name of the active space
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The name of the active space if one is set
    pub fn get_active_space_name(&self) -> Option<&str> {
        self.active_space.as_deref()
    }

    /// Gets a space configuration by name or path.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the space to retrieve
    ///
    /// # Returns
    ///
    /// - `Option<&SpaceConfig>` - The space configuration if found by name or path
    pub fn get_space_config(&self, name_or_path: &str) -> Option<&SpaceConfig> {
        if let Some(config) = self.spaces.get(name_or_path) {
            return Some(config);
        }

        let path = PathBuf::from(name_or_path);
        self.spaces
            .iter()
            .find(|(_, config)| config.path == path)
            .map(|(_, config)| config)
    }

    /// Checks if a space with the given path is already registered.
    ///
    /// # Arguments
    ///
    /// - `path` (`&std::path::Path`) - The path to check
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a sapce with this path is registered, `false` otherwise
    pub fn is_space_registered(&self, path: &std::path::Path) -> bool {
        self.spaces.values().any(|config| config.path == path)
    }

    /// Sets the active space by name or path.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the space to set as active
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if a space with the given name or path exists and was set as active
    ///
    /// # Errors
    ///
    /// Returns an error if no space with the given name or path exists or the configuration could not be saved
    pub fn set_active_space(&mut self, name_or_path: &str) -> Result<()> {
        if self.spaces.contains_key(name_or_path) {
            self.active_space = Some(name_or_path.to_string());
            return self.save();
        }

        let path = PathBuf::from(name_or_path);
        if let Some((space_name, _)) = self.spaces.iter().find(|(_, config)| config.path == path) {
            self.active_space = Some(space_name.clone());
            return self.save();
        }

        miette::bail!(
            "Space '{}' not found: not a name or path to a registered space",
            name_or_path
        )
    }

    /// Unregisters a space from the configuration by name or path.
    ///
    /// If the removed space was active, the alphabetically first remaining space
    /// will become active. If no spaces remain, the active space will be set to None.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the space to remove
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if the space was removed and saved successfully
    ///
    /// # Errors
    ///
    /// Returns an error if no space with the given name or path exists or the configuration could not be saved
    pub fn unregister_space(&mut self, name_or_path: &str) -> Result<()> {
        let space_name = if self.spaces.contains_key(name_or_path) {
            name_or_path.to_string()
        } else {
            let path = PathBuf::from(name_or_path);
            self.spaces
                .iter()
                .find(|(_, config)| config.path == path)
                .map(|(name, _)| name.clone())
                .with_context(|| {
                    format!(
                        "Space '{}' not found: not a name or path to a registered space",
                        name_or_path
                    )
                })?
        };

        self.spaces.remove(&space_name);

        // FIXME: If we removed the active space, pick a new one
        if self.active_space.as_deref() == Some(space_name.as_str()) {
            self.active_space = self.spaces.keys().min().map(|k| k.clone());
        }

        self.save()
    }

    /// Returns the number of registered spaces.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of spaces in the configuration
    pub fn space_count(&self) -> usize {
        self.spaces.len()
    }

    /// Returns a vector of all registered spaces (name, config).
    ///
    /// # Returns
    ///
    /// - `Vec<(String, SpaceConfig)>` - Vector of space names and their configurations
    pub fn all_spaces(&self) -> Vec<(String, SpaceConfig)> {
        self.spaces
            .iter()
            .map(|(name, config)| (name.clone(), config.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {}

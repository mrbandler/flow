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

use crate::graph::Graph;

const APP_NAME: &str = "flow";
const CONFIG_NAME: &str = "flow";

/// Main configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    graphs: HashMap<String, GraphConfig>,
    #[serde(default)]
    active_graph: Option<String>,
}

/// Graph configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    pub path: PathBuf,
}

/// Default configuration.
impl Default for Config {
    fn default() -> Self {
        Self {
            graphs: HashMap::new(),
            active_graph: None,
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

    /// Adds a graph to the configuration
    ///
    /// # Arguments
    ///
    /// - `graph` (`&Graph`) - The graph to add to the configuration
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if the graph was added and saved successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration could not be saved to disk
    pub fn add_graph(&mut self, graph: &Graph) -> Result<()> {
        // Canonicalize the path to ensure we always store absolute paths
        let canonical_path = graph
            .path()
            .canonicalize()
            .into_diagnostic()
            .context(format!(
                "Failed to canonicalize path: {}",
                graph.path().display()
            ))?;

        let entry = GraphConfig {
            path: canonical_path,
        };

        let graph_name = graph.name().to_owned();
        self.graphs.insert(graph_name.clone(), entry);

        // Set as active if it's the first graph
        if self.graphs.len() == 1 {
            self.active_graph = Some(graph_name);
        }

        self.save()
    }

    /// Gets the active graph configuration
    ///
    /// # Returns
    ///
    /// - `Option<&GraphConfig>` - The active graph configuration if one is set and exists
    pub fn get_active_graph(&self) -> Option<&GraphConfig> {
        self.active_graph
            .as_ref()
            .and_then(|name| self.graphs.get(name))
    }

    /// Gets the name of the active graph
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The name of the active graph if one is set
    pub fn get_active_graph_name(&self) -> Option<&str> {
        self.active_graph.as_deref()
    }

    /// Gets a graph configuration by name or path.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the graph to retrieve
    ///
    /// # Returns
    ///
    /// - `Option<&GraphConfig>` - The graph configuration if found by name or path
    pub fn get_graph_config(&self, name_or_path: &str) -> Option<&GraphConfig> {
        if let Some(config) = self.graphs.get(name_or_path) {
            return Some(config);
        }

        let path = PathBuf::from(name_or_path);
        self.graphs
            .iter()
            .find(|(_, config)| config.path == path)
            .map(|(_, config)| config)
    }

    /// Checks if a graph with the given path is already registered.
    ///
    /// # Arguments
    ///
    /// - `path` (`&std::path::Path`) - The path to check
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a graph with this path is registered, `false` otherwise
    pub fn is_graph_registered(&self, path: &std::path::Path) -> bool {
        self.graphs.values().any(|config| config.path == path)
    }

    /// Sets the active graph by name or path.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the graph to set as active
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if a graph with the given name or path exists and was set as active
    ///
    /// # Errors
    ///
    /// Returns an error if no graph with the given name or path exists or the configuration could not be saved
    pub fn set_active_graph(&mut self, name_or_path: &str) -> Result<()> {
        if self.graphs.contains_key(name_or_path) {
            self.active_graph = Some(name_or_path.to_string());
            return self.save();
        }

        let path = PathBuf::from(name_or_path);
        if let Some((graph_name, _)) = self.graphs.iter().find(|(_, config)| config.path == path) {
            self.active_graph = Some(graph_name.clone());
            return self.save();
        }

        miette::bail!(
            "Graph '{}' not found: not a name or path to a registered graph",
            name_or_path
        )
    }

    /// Removes a graph from the configuration by name or path.
    ///
    /// If the removed graph was active, the alphabetically first remaining graph
    /// will become active. If no graphs remain, the active graph will be set to None.
    ///
    /// # Arguments
    ///
    /// - `name_or_path` (`&str`) - The name or path of the graph to remove
    ///
    /// # Returns
    ///
    /// - `Result<()>` - Ok if the graph was removed and saved successfully
    ///
    /// # Errors
    ///
    /// Returns an error if no graph with the given name or path exists or the configuration could not be saved
    pub fn remove_graph(&mut self, name_or_path: &str) -> Result<()> {
        let graph_name = if self.graphs.contains_key(name_or_path) {
            name_or_path.to_string()
        } else {
            let path = PathBuf::from(name_or_path);
            self.graphs
                .iter()
                .find(|(_, config)| config.path == path)
                .map(|(name, _)| name.clone())
                .with_context(|| {
                    format!(
                        "Graph '{}' not found: not a name or path to a registered graph",
                        name_or_path
                    )
                })?
        };

        self.graphs.remove(&graph_name);

        // FIXME: If we removed the active graph, pick a new one
        if self.active_graph.as_deref() == Some(graph_name.as_str()) {
            self.active_graph = self.graphs.keys().min().map(|k| k.clone());
        }

        self.save()
    }

    /// Returns the number of registered graphs.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of graphs in the configuration
    pub fn graph_count(&self) -> usize {
        self.graphs.len()
    }

    /// Returns a vector of all registered graphs (name, config).
    ///
    /// # Returns
    ///
    /// - `Vec<(String, GraphConfig)>` - Vector of graph names and their configurations
    pub fn all_graphs(&self) -> Vec<(String, GraphConfig)> {
        self.graphs
            .iter()
            .map(|(name, config)| (name.clone(), config.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {}

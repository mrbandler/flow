//! Flow Application Library
//!
//! This crate provides the shared application logic for Flow, serving as the
//! bridge between the core library and various user interfaces (CLI, TUI, Desktop).
//!
//! # Architecture
//!
//! The `flow-app` crate sits between `flow-core` and the UI crates:
//!
//! ```text
//! ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐
//! │   flow-cli  │  │  flow-tui   │  │  flow-desktop   │
//! └──────┬──────┘  └──────┬──────┘  └────────┬────────┘
//!        │                │                  │
//!        └────────────────┼──────────────────┘
//!                         │
//!                  ┌──────▼──────┐
//!                  │  flow-app   │
//!                  └──────┬──────┘
//!                         │
//!                  ┌──────▼──────┐
//!                  │  flow-core  │
//!                  └─────────────┘
//! ```
//!
//! # Responsibilities
//!
//! - Application state management
//! - Business logic that spans multiple core operations
//! - Event handling and coordination
//! - Shared UI-agnostic behaviors
//!
//! # Example
//!
//! ```rust,ignore
//! use flow_app::App;
//!
//! let app = App::new()?;
//! app.open_graph("my-notes")?;
//! ```

#![doc(html_root_url = "https://docs.rs/flow-app/0.1.0")]

// Re-export core types for convenience
pub use flow_core::{Config, Graph, Space, SpaceConfig};

/// Application state and coordinator
///
/// The `App` struct manages the overall application state and provides
/// high-level operations that coordinate between different core components.
#[derive(Debug)]
pub struct App {
    /// The current configuration
    config: Config,
    /// The currently active graph, if any
    active_graph: Option<Graph>,
}

impl App {
    /// Creates a new application instance
    ///
    /// This loads the configuration from disk and sets up the initial state.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded.
    pub fn new() -> miette::Result<Self> {
        let config = Config::load()?;
        let active_graph = None;

        Ok(Self {
            config,
            active_graph,
        })
    }

    /// Returns a reference to the current configuration
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns a mutable reference to the current configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Returns a reference to the active graph, if any
    #[must_use]
    pub fn active_graph(&self) -> Option<&Graph> {
        self.active_graph.as_ref()
    }

    /// Returns a mutable reference to the active graph, if any
    pub fn active_graph_mut(&mut self) -> Option<&mut Graph> {
        self.active_graph.as_mut()
    }

    /// Opens a graph by name or path
    ///
    /// If the graph is found in the configuration, it is loaded and set as active.
    ///
    /// # Arguments
    ///
    /// * `name_or_path` - The name of a registered graph or a path to a graph directory
    ///
    /// # Errors
    ///
    /// Returns an error if the graph cannot be found or loaded.
    pub fn open_graph(&mut self, name_or_path: &str) -> miette::Result<&Graph> {
        use miette::Context;
        use std::path::PathBuf;

        let graph = if let Some(space_config) = self.config.get_space_config(name_or_path) {
            Graph::load(&space_config.path).with_context(|| {
                format!(
                    "Failed to load graph from '{}'",
                    space_config.path.display()
                )
            })?
        } else {
            let path = PathBuf::from(name_or_path);
            Graph::load(&path)
                .with_context(|| format!("Failed to load graph from '{}'", path.display()))?
        };

        self.active_graph = Some(graph);
        // Safe to unwrap since we just set it
        Ok(self.active_graph.as_ref().expect("graph was just set"))
    }

    /// Closes the active graph
    pub fn close_graph(&mut self) {
        self.active_graph = None;
    }

    /// Checks if there is an active graph
    #[must_use]
    pub fn has_active_graph(&self) -> bool {
        self.active_graph.is_some()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: Config::default(),
            active_graph: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_default_has_no_active_graph() {
        let app = App::default();
        assert!(!app.has_active_graph());
        assert!(app.active_graph().is_none());
    }

    #[test]
    fn test_close_graph_clears_active() {
        let mut app = App::default();
        app.close_graph();
        assert!(!app.has_active_graph());
    }
}

//! Flow Core Library
//!
//! This crate provides the core functionality for Flow, including:
//!
//! - Configuration management (`config`)
//! - Space/Graph storage and operations (`space`, `graph`)
//!
//! # Overview
//!
//! Flow is a note-taking application that stores knowledge in "spaces" (also called "graphs").
//! Each space is a directory containing markdown files organized as an outliner with
//! bidirectional linking support.
//!
//! # Example
//!
//! ```rust,ignore
//! use flow_core::{Config, Graph};
//! use std::path::Path;
//!
//! // Load configuration
//! let mut config = Config::load()?;
//!
//! // Initialize a new graph
//! let graph = Graph::init(Path::new("./my-notes"), Some(&"My Notes".to_string()))?;
//!
//! // Register it in the configuration
//! config.register_space(&graph)?;
//! ```

#![doc(html_root_url = "https://docs.rs/flow-core/0.1.0")]

pub mod config;

/// Space module - storage and operations for Flow knowledge bases.
///
/// A space represents a directory containing interconnected markdown notes.
/// See [`Space`] for the main type.
pub mod space;

// Re-export main types at crate root for convenience
pub use config::{Config, SpaceConfig};
pub use space::Space;

/// Type alias for Space - "Graph" is the user-facing terminology
///
/// In Flow's documentation and CLI, we use "graph" to refer to a knowledge base,
/// as it represents a graph of interconnected notes. Internally, we use "Space"
/// as it better represents the storage/workspace concept.
pub type Graph = Space;

/// Module providing the Graph type alias
///
/// This module exists to allow importing `flow_core::graph::Graph` for code
/// that prefers explicit module paths.
pub mod graph {
    //! Graph module - provides the Graph type for knowledge bases.
    //!
    //! A Graph (internally called Space) represents a Flow knowledge base,
    //! which is a directory containing interconnected markdown notes.

    pub use crate::Space as Graph;
}

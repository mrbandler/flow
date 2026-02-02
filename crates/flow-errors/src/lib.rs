//! Error types for the Flow notes and outliner system.
//!
//! This crate provides the error types used throughout the Flow ecosystem.
//! Errors use [`miette`] for rich diagnostic output with error codes,
//! help text, and source code context.
//!
//! # Overview
//!
//! This crate consolidates error types from across the Flow crates:
//!
//! - [`SpaceError`] - Errors related to space operations (initialization, loading, registration)
//! - [`CliError`] - Errors specific to CLI operations
//! - [`IoError`] - Wrapper for filesystem I/O errors
//!
//! # Error Handling
//!
//! All errors in this crate are compatible with [`miette`]'s diagnostic
//! system, which provides rich error reporting in CLI applications.
//!
//! # Examples
//!
//! ```
//! use flow_errors::SpaceError;
//! use std::path::PathBuf;
//!
//! // Errors can be created directly
//! let error = SpaceError::NotFound(PathBuf::from("/nonexistent/path"));
//!
//! // They implement Display for human-readable messages
//! println!("Error: {}", error);
//! ```

mod cli;
mod io;
mod space;
mod theme;

pub use cli::CliError;
pub use io::IoError;
pub use space::SpaceError;
pub use theme::ThemeError;

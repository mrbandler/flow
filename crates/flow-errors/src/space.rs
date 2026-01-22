//! Error types for space operations.
//!
//! This module defines the error types used when working with Flow spaces.
//! Errors use [`miette`] for rich diagnostic output with error codes,
//! help text, and source code context.

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

/// Errors that can occur when working with spaces.
///
/// Each variant provides diagnostic information through [`miette`],
/// including error codes, help text, and documentation links.
#[derive(Debug, Error, Diagnostic)]
pub enum SpaceError {
    /// The specified path does not exist on the filesystem.
    #[error("Path does not exist: {0}")]
    #[diagnostic(
        code(flow::path_not_found),
        url(docsrs),
        help("Make sure the directory exists before initializing a space")
    )]
    NotFound(PathBuf),

    /// A space already exists at the specified path (`.flow` directory found).
    #[error("A space already exists at: {0}")]
    #[diagnostic(
        code(flow::already_exists),
        url(docsrs),
        help("Use `flow open` to open the existing space, or choose a different path")
    )]
    AlreadyExists(PathBuf),

    /// The path exists but is not a directory.
    #[error("Path is not a directory: {0}")]
    #[diagnostic(code(flow::not_a_directory), url(docsrs), help("Make sure the path is a directory"))]
    NotADirectory(PathBuf),

    /// The directory contains files or subdirectories.
    #[error("Directory is not empty: {0}")]
    #[diagnostic(
        code(flow::directory_not_empty),
        url(docsrs),
        help("Initialize a space in an empty directory, or use a different path")
    )]
    DirectoryNotEmpty(PathBuf),

    /// A space with the given name is already registered in the config.
    #[error("A space with the name '{0}' is already registered")]
    #[diagnostic(
        code(flow::space_already_registered),
        url(docsrs),
        help("Use a different name, or unregister the existing space first")
    )]
    AlreadyRegistered(String),

    /// A space at the given path is already registered under a different name.
    #[error("A space at path '{0}' is already registered")]
    #[diagnostic(
        code(flow::path_already_registered),
        url(docsrs),
        help("This space is already registered under a different name")
    )]
    PathAlreadyRegistered(PathBuf),

    /// The specified space was not found in the configuration.
    #[error("Space not registered: {0}")]
    #[diagnostic(
        code(flow::not_registered),
        url(docsrs),
        help("Register the space first with `flow register`")
    )]
    NotRegistered(String),
}

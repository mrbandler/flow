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
/// This enum covers all error conditions that may arise during space
/// initialization, loading, and filesystem operations. Each variant
/// provides detailed diagnostic information through [`miette`].
///
/// # Variants
///
/// | Variant | Error Code | Description |
/// |---------|------------|-------------|
/// | [`NotFound`](SpaceError::NotFound) | `flow::path_not_found` | Path does not exist |
/// | [`NotADirectory`](SpaceError::NotADirectory) | `flow::not_a_directory` | Path is not a directory |
/// | [`AlreadyExists`](SpaceError::AlreadyExists) | `flow::already_exists` | Space already exists |
/// | [`DirectoryNotEmpty`](SpaceError::DirectoryNotEmpty) | `flow::directory_not_empty` | Directory has contents |
/// | [`SpaceAlreadyRegistered`](SpaceError::SpaceAlreadyRegistered) | `flow::space_already_registered` | Name already in use |
/// | [`SpacePathAlreadyRegistered`](SpaceError::SpacePathAlreadyRegistered) | `flow::space_path_already_registered` | Path already registered |
/// | [`SpaceNotRegistered`](SpaceError::SpaceNotRegistered) | `flow::space_not_registered` | Space not found in config |
///
/// # Examples
///
/// Matching on specific error variants:
///
/// ```
/// use flow_errors::SpaceError;
/// use std::path::PathBuf;
///
/// fn handle_error(error: SpaceError) {
///     match error {
///         SpaceError::NotFound(path) => {
///             eprintln!("Path not found: {}", path.display());
///         }
///         SpaceError::AlreadyExists(path) => {
///             eprintln!("Space already exists at: {}", path.display());
///         }
///         _ => eprintln!("An error occurred: {}", error),
///     }
/// }
/// ```
#[derive(Debug, Error, Diagnostic)]
pub enum SpaceError {
    /// The specified path does not exist.
    ///
    /// This error occurs when attempting to initialize or load a space
    /// from a path that does not exist on the filesystem.
    ///
    /// # Error Code
    ///
    /// `flow::path_not_found`
    ///
    /// # Fields
    ///
    /// * `0` - The path that was not found.
    #[error("Path does not exist: {0}")]
    #[diagnostic(
        code(flow::path_not_found),
        url(docsrs),
        help("Make sure the directory exists before initializing a space")
    )]
    NotFound(PathBuf),

    /// A space already exists at the specified path.
    ///
    /// This error occurs when attempting to initialize a new space in
    /// a directory that already contains a `.flow` directory, indicating
    /// an existing space.
    ///
    /// # Error Code
    ///
    /// `flow::already_exists`
    ///
    /// # Fields
    ///
    /// * `0` - The path where the space already exists.
    #[error("A space already exists at: {0}")]
    #[diagnostic(
        code(flow::already_exists),
        url(docsrs),
        help("Use `flow open` to open the existing space, or choose a different path")
    )]
    AlreadyExists(PathBuf),

    /// The specified path is not a directory.
    ///
    /// This error occurs when a path exists but is a file rather than
    /// a directory. Spaces can only be initialized in directories.
    ///
    /// # Error Code
    ///
    /// `flow::not_a_directory`
    ///
    /// # Fields
    ///
    /// * `0` - The path that is not a directory.
    #[error("Path is not a directory: {0}")]
    #[diagnostic(code(flow::not_a_directory), url(docsrs), help("Make sure the path is a directory"))]
    NotADirectory(PathBuf),

    /// The directory is not empty.
    ///
    /// This error occurs when attempting to initialize a space in a
    /// directory that contains files or subdirectories. Spaces should
    /// be initialized in empty directories to avoid conflicts.
    ///
    /// # Error Code
    ///
    /// `flow::directory_not_empty`
    ///
    /// # Fields
    ///
    /// * `0` - The path to the non-empty directory.
    #[error("Directory is not empty: {0}")]
    #[diagnostic(
        code(flow::directory_not_empty),
        url(docsrs),
        help("Initialize a space in an empty directory, or use a different path")
    )]
    DirectoryNotEmpty(PathBuf),

    /// A space with the given name is already registered.
    ///
    /// This error occurs when attempting to register a space with a name
    /// that is already in use by another registered space.
    ///
    /// # Error Code
    ///
    /// `flow::already_registered`
    ///
    /// # Fields
    ///
    /// * `0` - The name that is already registered.
    #[error("A space with the name '{0}' is already registered")]
    #[diagnostic(
        code(flow::space_already_registered),
        url(docsrs),
        help("Use a different name, or unregister the existing space first")
    )]
    AlreadyRegistered(String),

    /// A space at the given path is already registered.
    ///
    /// This error occurs when attempting to register a space at a path
    /// that is already registered under a different name.
    ///
    /// # Error Code
    ///
    /// `flow::space_path_already_registered`
    ///
    /// # Fields
    ///
    /// * `0` - The path that is already registered.
    #[error("A space at path '{0}' is already registered")]
    #[diagnostic(
        code(flow::path_already_registered),
        url(docsrs),
        help("This space is already registered under a different name")
    )]
    PathAlreadyRegistered(PathBuf),

    /// The specified space is not registered.
    ///
    /// This error occurs when attempting to operate on a space that
    /// has not been registered in the configuration.
    ///
    /// # Error Code
    ///
    /// `flow::space_not_registered`
    ///
    /// # Fields
    ///
    /// * `0` - The locator used to find the space (as a string).
    #[error("Space not registered: {0}")]
    #[diagnostic(
        code(flow::not_registered),
        url(docsrs),
        help("Register the space first with `flow register`")
    )]
    NotRegistered(String),
}

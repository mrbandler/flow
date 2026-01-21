//! Error types for flow-core operations.
//!
//! This module defines the error types used throughout the crate.
//! Errors use [`miette`] for rich diagnostic output with error codes,
//! help text, and source code context.
//!
//! # Overview
//!
//! The [`Error`] enum represents all possible errors that can occur when
//! working with Flow spaces and filesystems. Each variant includes:
//!
//! - A human-readable error message
//! - A unique error code (e.g., `flow::io_error`)
//! - Contextual help text to guide users toward a solution
//!
//! # Error Handling
//!
//! All errors in this crate are compatible with [`miette`]'s diagnostic
//! system, which provides rich error reporting in CLI applications.
//!
//! # Examples
//!
//! ```
//! use flow_core::Error;
//! use std::path::PathBuf;
//!
//! // Errors can be created directly
//! let error = Error::PathNotFound(PathBuf::from("/nonexistent/path"));
//!
//! // They implement Display for human-readable messages
//! println!("Error: {}", error);
//! ```

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

use crate::space::Locator;

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
/// | [`Io`](Error::Io) | `flow::io_error` | Low-level filesystem errors |
/// | [`PathNotFound`](Error::PathNotFound) | `flow::path_not_found` | Path does not exist |
/// | [`NotADirectory`](Error::NotADirectory) | `flow::not_a_directory` | Path is not a directory |
/// | [`AlreadyExists`](Error::AlreadyExists) | `flow::already_exists` | Space already exists |
/// | [`DirectoryNotEmpty`](Error::DirectoryNotEmpty) | `flow::directory_not_empty` | Directory has contents |
///
/// # Examples
///
/// Matching on specific error variants:
///
/// ```
/// use flow_core::Error;
/// use std::path::PathBuf;
///
/// fn handle_error(error: Error) {
///     match error {
///         Error::PathNotFound(path) => {
///             eprintln!("Path not found: {}", path.display());
///         }
///         Error::AlreadyExists(path) => {
///             eprintln!("Space already exists at: {}", path.display());
///         }
///         _ => eprintln!("An error occurred: {}", error),
///     }
/// }
/// ```
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    /// A filesystem operation failed.
    ///
    /// This variant wraps low-level I/O errors from the operating system,
    /// such as permission denied, disk full, or network errors when
    /// accessing remote filesystems.
    ///
    /// # Error Code
    ///
    /// `flow::io_error`
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_core::Error;
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    /// let error: Error = io_error.into();
    /// ```
    #[error("Filesystem error: {0}")]
    #[diagnostic(code(flow::io_error))]
    Io(#[from] std::io::Error),

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
    /// `flow::space_already_registered`
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
    SpaceAlreadyRegistered(String),

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
        code(flow::space_path_already_registered),
        url(docsrs),
        help("This space is already registered under a different name")
    )]
    SpacePathAlreadyRegistered(PathBuf),

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
    /// * `0` - The locator used to find the space.
    #[error("Space not registered: {0}")]
    #[diagnostic(
        code(flow::space_not_registered),
        url(docsrs),
        help("Register the space first with `flow register`")
    )]
    SpaceNotRegistered(Locator),
}

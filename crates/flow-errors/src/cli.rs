//! Error types for CLI operations.
//!
//! This module defines the error types used by the Flow CLI.

use miette::Diagnostic;
use thiserror::Error;

/// Errors that can occur during CLI operations.
///
/// This enum covers error conditions specific to the command-line interface,
/// such as missing arguments or configuration issues.
#[derive(Debug, Error, Diagnostic)]
pub enum CliError {
    /// A required argument was not provided.
    ///
    /// This error occurs when a command requires an argument that
    /// was not supplied by the user and could not be inferred.
    ///
    /// # Error Code
    ///
    /// `flow::missing_argument`
    ///
    /// # Fields
    ///
    /// * `0` - The name of the missing argument.
    #[error("Missing argument: {0}")]
    #[diagnostic(code(flow::missing_argument), url(docsrs))]
    MissingArgument(String),

    /// No active space is set.
    ///
    /// This error occurs when a command requires an active space
    /// but none has been set in the configuration.
    ///
    /// # Error Code
    ///
    /// `flow::no_active_space`
    #[error("There is no active space set")]
    #[diagnostic(
        code(flow::no_active_space),
        url(docsrs),
        help("Use --space to specify one specifically or register one with `flow register`.")
    )]
    NoActiveSpace,

    /// No spaces are registered.
    ///
    /// This error occurs when a command requires at least one registered
    /// space but none have been registered yet.
    ///
    /// # Error Code
    ///
    /// `flow::no_spaces_registered`
    #[error("No spaces are registered")]
    #[diagnostic(
        code(flow::no_spaces_registered),
        url(docsrs),
        help("Register a space first with `flow space init` or `flow space register`.")
    )]
    NoSpacesRegistered,
}

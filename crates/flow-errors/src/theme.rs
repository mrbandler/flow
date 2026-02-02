//! Error types for theme operations.
//!
//! This module defines the error types used when loading and parsing themes.
//! Errors use [`miette`] for rich diagnostic output with error codes and help text.

use miette::Diagnostic;
use thiserror::Error;

/// Errors that can occur when working with themes.
///
/// Each variant provides diagnostic information through [`miette`],
/// including error codes and help text.
#[derive(Debug, Error, Diagnostic)]
pub enum ThemeError {
    /// The hex color string is not valid.
    #[error("Invalid hex color: {0}")]
    #[diagnostic(
        code(flow::theme::invalid_hex_color),
        help("Hex colors must be in the format '#RRGGBB' or 'RRGGBB'")
    )]
    InvalidHexColor(String),

    /// The requested built-in theme does not exist.
    #[error("Unknown built-in theme: {0}")]
    #[diagnostic(
        code(flow::theme::unknown_builtin),
        help("Available themes: flow, dracula, nord, gruvbox-dark, solarized-dark, catppuccin-mocha, tokyo-night, one-dark, monokai, rose-pine")
    )]
    UnknownBuiltIn(String),

    /// Failed to load the theme file from disk.
    #[error("Failed to load theme file: {0}")]
    #[diagnostic(
        code(flow::theme::file_load_failed),
        help("Check that the file exists and is readable")
    )]
    FileLoadFailed(String),

    /// Failed to fetch the theme from a URL.
    #[error("Failed to fetch theme from URL: {0}")]
    #[diagnostic(
        code(flow::theme::fetch_failed),
        help("Check your network connection and that the URL is accessible")
    )]
    FetchFailed(String),

    /// The theme YAML is malformed or missing required fields.
    #[error("Invalid theme YAML: {0}")]
    #[diagnostic(
        code(flow::theme::invalid_yaml),
        help("Theme files must be valid base16 YAML with all 16 color values (base00-base0F)")
    )]
    InvalidYaml(String),
}

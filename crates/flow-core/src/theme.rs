//! Theme system for Flow using base16 color schemes.
//!
//! This module provides a configurable theming system based on the base16 color scheme
//! specification. Themes can be loaded from:
//!
//! - Built-in themes (e.g., "dracula", "nord", "gruvbox-dark")
//! - Local YAML files (e.g., "~/.config/flow/themes/my-theme.yaml")
//! - Remote URLs (e.g., `https://raw.githubusercontent.com/.../theme.yaml`)
//!
//! # Base16 Color Scheme
//!
//! Base16 defines 16 colors (base00-base0F) that are mapped semantically:
//!
//! - **base00-base07**: Shades from darkest to lightest (backgrounds and foregrounds)
//! - **base08**: Red (errors, deletions)
//! - **base09**: Orange (integers, constants)
//! - **base0A**: Yellow (warnings, classes)
//! - **base0B**: Green (success, strings)
//! - **base0C**: Cyan (info, support)
//! - **base0D**: Blue (primary, functions)
//! - **base0E**: Magenta (keywords)
//! - **base0F**: Brown (deprecated, embedded)
//!
//! # Examples
//!
//! ```no_run
//! use flow_core::theme::{resolve, Base16Palette};
//!
//! # async fn example() -> miette::Result<()> {
//! // Load a built-in theme
//! let palette = resolve(Some("dracula")).await?;
//!
//! // Load from a file
//! let palette = resolve(Some("~/.config/flow/themes/custom.yaml")).await?;
//!
//! // Load from a URL
//! let palette = resolve(Some("https://example.com/theme.yaml")).await?;
//!
//! // Use the default "flow" theme
//! let palette = resolve(None).await?;
//! # Ok(())
//! # }
//! ```

pub mod builtin;

use std::path::Path;

use flow_errors::ThemeError;
use miette::Result;
use serde::{Deserialize, Serialize};

/// A hex color value.
///
/// Stores a color as a hex string (with or without leading `#`).
/// The string should contain 6 hex digits representing RGB values.
///
/// # Examples
///
/// ```
/// use flow_core::theme::HexColor;
///
/// let color = HexColor::new("282a36");
/// let (r, g, b) = color.to_rgb().unwrap();
/// assert_eq!((r, g, b), (40, 42, 54));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexColor(pub String);

impl HexColor {
    /// Creates a new `HexColor` from a hex string.
    ///
    /// The string may optionally include a leading `#`.
    #[must_use]
    pub fn new(hex: impl Into<String>) -> Self {
        Self(hex.into())
    }

    /// Converts the hex color to RGB values.
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is not a valid 6-digit hex color.
    pub fn to_rgb(&self) -> Result<(u8, u8, u8), ThemeError> {
        let hex = self.0.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(ThemeError::InvalidHexColor(self.0.clone()));
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ThemeError::InvalidHexColor(self.0.clone()))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ThemeError::InvalidHexColor(self.0.clone()))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ThemeError::InvalidHexColor(self.0.clone()))?;

        Ok((r, g, b))
    }
}

/// A base16 color palette with 16 semantic colors.
///
/// The base16 scheme provides a consistent set of colors that can be
/// applied across different UI elements. The colors are:
///
/// - **base00-base07**: Background and foreground shades
/// - **base08-base0F**: Accent colors for syntax and UI elements
///
/// # Serialization
///
/// The palette serializes to/from YAML in the standard base16 format:
///
/// ```yaml
/// scheme: "Dracula"
/// author: "Author Name"
/// base00: "282a36"
/// base01: "3a3c4e"
/// # ... remaining colors
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Base16Palette {
    /// Optional scheme name.
    #[serde(default)]
    pub scheme: Option<String>,

    /// Optional author name.
    #[serde(default)]
    pub author: Option<String>,

    /// Default background (darkest).
    pub base00: HexColor,
    /// Lighter background (status bars, line numbers).
    pub base01: HexColor,
    /// Selection background.
    pub base02: HexColor,
    /// Comments, invisibles.
    pub base03: HexColor,
    /// Dark foreground (status bars).
    pub base04: HexColor,
    /// Default foreground.
    pub base05: HexColor,
    /// Light foreground.
    pub base06: HexColor,
    /// Lightest foreground.
    pub base07: HexColor,
    /// Red (errors, deletions).
    pub base08: HexColor,
    /// Orange (integers, constants).
    pub base09: HexColor,
    /// Yellow (warnings, classes).
    #[serde(rename = "base0A")]
    pub base0a: HexColor,
    /// Green (success, strings).
    #[serde(rename = "base0B")]
    pub base0b: HexColor,
    /// Cyan (info, support).
    #[serde(rename = "base0C")]
    pub base0c: HexColor,
    /// Blue (primary, functions).
    #[serde(rename = "base0D")]
    pub base0d: HexColor,
    /// Magenta (keywords).
    #[serde(rename = "base0E")]
    pub base0e: HexColor,
    /// Brown (deprecated, embedded).
    #[serde(rename = "base0F")]
    pub base0f: HexColor,
}

/// Resolves a theme string to a `Base16Palette`.
///
/// The theme string can be:
/// - A built-in theme name (e.g., "dracula", "nord")
/// - A file path (e.g., "~/.config/flow/themes/my-theme.yaml")
/// - A URL (e.g., `https://example.com/theme.yaml`)
/// - `None` to use the default "flow" theme
///
/// # Detection Logic
///
/// The type of theme source is detected automatically:
/// - Starts with `http://` or `https://` → URL
/// - Ends with `.yaml`/`.yml` or contains path separators → File
/// - Otherwise → Built-in theme name
///
/// # Errors
///
/// Returns an error if:
/// - The built-in theme name is not recognized
/// - The file cannot be read
/// - The URL cannot be fetched
/// - The YAML is invalid or missing required fields
///
/// # Examples
///
/// ```no_run
/// use flow_core::theme::resolve;
///
/// # async fn example() -> miette::Result<()> {
/// // Load built-in theme
/// let palette = resolve(Some("dracula")).await?;
///
/// // Load from file
/// let palette = resolve(Some("/path/to/theme.yaml")).await?;
///
/// // Use default
/// let palette = resolve(None).await?;
/// # Ok(())
/// # }
/// ```
pub async fn resolve(theme: Option<&str>) -> Result<Base16Palette, ThemeError> {
    let theme = theme.unwrap_or("flow");

    if theme.starts_with("http://") || theme.starts_with("https://") {
        fetch_from_url(theme).await
    } else if is_file_path(theme) {
        load_from_file(Path::new(theme)).await
    } else {
        builtin::get(theme)
    }
}

/// Determines if the theme string looks like a file path.
fn is_file_path(theme: &str) -> bool {
    let path = std::path::Path::new(theme);
    let has_yaml_extension = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml"));

    has_yaml_extension
        || theme.contains('/')
        || theme.contains('\\')
        || theme.starts_with('~')
        || theme.starts_with('.')
}

/// Loads a theme from a local YAML file.
async fn load_from_file(path: &Path) -> Result<Base16Palette, ThemeError> {
    // Expand ~ to home directory
    let expanded_path = if path.starts_with("~") {
        dirs_path().map_or_else(
            || path.to_path_buf(),
            |home| home.join(path.strip_prefix("~").unwrap_or(path)),
        )
    } else {
        path.to_path_buf()
    };

    let content = tokio::fs::read_to_string(&expanded_path)
        .await
        .map_err(|e| ThemeError::FileLoadFailed(format!("{}: {e}", expanded_path.display())))?;

    parse_yaml(&content)
}

/// Fetches a theme from a URL.
async fn fetch_from_url(url: &str) -> Result<Base16Palette, ThemeError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| ThemeError::FetchFailed(format!("{url}: {e}")))?;

    if !response.status().is_success() {
        return Err(ThemeError::FetchFailed(format!("{url}: HTTP {}", response.status())));
    }

    let content = response
        .text()
        .await
        .map_err(|e| ThemeError::FetchFailed(format!("{url}: {e}")))?;

    parse_yaml(&content)
}

/// Parses YAML content into a `Base16Palette`.
fn parse_yaml(content: &str) -> Result<Base16Palette, ThemeError> {
    serde_yaml::from_str(content).map_err(|e| ThemeError::InvalidYaml(e.to_string()))
}

/// Gets the user's home directory.
fn dirs_path() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .ok()
            .map(std::path::PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(std::path::PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_to_rgb() {
        let color = HexColor::new("282a36");
        let (r, g, b) = color.to_rgb().unwrap();
        assert_eq!((r, g, b), (40, 42, 54));
    }

    #[test]
    fn test_hex_color_with_hash() {
        let color = HexColor::new("#ff5555");
        let (r, g, b) = color.to_rgb().unwrap();
        assert_eq!((r, g, b), (255, 85, 85));
    }

    #[test]
    fn test_hex_color_invalid() {
        let color = HexColor::new("invalid");
        assert!(color.to_rgb().is_err());
    }

    #[test]
    fn test_is_file_path() {
        assert!(is_file_path("~/themes/my-theme.yaml"));
        assert!(is_file_path("./theme.yaml"));
        assert!(is_file_path("/path/to/theme.yml"));
        assert!(is_file_path("C:\\themes\\theme.yaml"));
        assert!(!is_file_path("dracula"));
        assert!(!is_file_path("nord"));
    }

    #[tokio::test]
    async fn test_resolve_builtin() {
        let palette = resolve(Some("flow")).await.unwrap();
        assert!(palette.scheme.is_some());
    }

    #[tokio::test]
    async fn test_resolve_default() {
        let palette = resolve(None).await.unwrap();
        assert_eq!(palette.scheme.as_deref(), Some("Flow"));
    }
}

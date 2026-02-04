//! Base16 color palette types.
//!
//! This module defines the core color types used by the theme system:
//! [`HexColor`] for individual colors and [`Base16Palette`] for the full
//! 16-color scheme.

use flow_errors::ThemeError;
use serde::{Deserialize, Serialize};

/// A hex color value.
///
/// Stores a color as a hex string (with or without leading `#`).
/// The string should contain 6 hex digits representing RGB values.
///
/// # Examples
///
/// ```
/// use flow_theme::HexColor;
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

impl Default for Base16Palette {
    fn default() -> Self {
        Self::builtin("flow").expect("flow is a built-in theme")
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
}

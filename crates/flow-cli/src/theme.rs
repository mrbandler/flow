//! Theme configuration for consistent CLI styling.
//!
//! This module provides a configurable theming system for the CLI using base16 color
//! schemes. The theme affects both `inquire` prompts and printer output.
//!
//! # Theme Resolution
//!
//! Themes are resolved from the user's config file (`~/.config/flow/config.json`):
//! - Built-in theme names (e.g., "dracula", "nord")
//! - Local YAML files (e.g., "~/.config/flow/themes/my-theme.yaml")
//! - Remote URLs (e.g., `https://example.com/theme.yaml`)
//! - Default "flow" theme if not specified
//!
//! # Usage
//!
//! The theme is initialized once at CLI startup via [`Context::load`](crate::context::Context::load),
//! which loads the palette from config and registers it for `inquire` prompts.
//! Commands access the theme through the context.

#![allow(dead_code)]

use crossterm::style::Color as CrosstermColor;
use flow_core::theme::{Base16Palette, HexColor};
use inquire::ui::{Attributes, Color as InquireColor, RenderConfig, StyleSheet, Styled};

/// Symbols used in the CLI theme.
///
/// These symbols are shared between the printer and inquire theme
/// for consistent visual styling across all CLI output.
pub mod symbols {
    /// Prompt indicator (used for questions).
    pub const PROMPT: &str = "?";
    /// Success indicator (checkmark).
    pub const SUCCESS: &str = "\u{2713}"; // ✓
    /// Error indicator (cross).
    pub const ERROR: &str = "\u{2717}"; // ✗
    /// Step/selection indicator (arrow).
    pub const STEP: &str = "\u{2192}"; // →
    /// Info indicator (bullet).
    pub const INFO: &str = "\u{2022}"; // •
    /// Warning indicator.
    pub const WARN: &str = "!";
    /// Debug indicator.
    pub const DEBUG: &str = "~";
    /// Heading indicator (section symbol).
    pub const HEADING: &str = "\u{00A7}"; // §
}

/// A theme instance containing a base16 color palette.
///
/// The theme converts base16 colors to the appropriate color types for
/// inquire prompts and crossterm terminal output.
#[derive(Debug, Clone)]
pub struct Theme {
    palette: Base16Palette,
}

impl Theme {
    /// Creates a new theme from a base16 palette.
    #[must_use]
    pub const fn new(palette: Base16Palette) -> Self {
        Self { palette }
    }

    /// Creates the inquire render configuration for this theme.
    #[must_use]
    pub fn render_config(&self) -> RenderConfig<'static> {
        RenderConfig {
            prompt_prefix: Styled::new(symbols::PROMPT).with_fg(self.inquire_primary()),
            answered_prompt_prefix: Styled::new(symbols::SUCCESS).with_fg(self.inquire_success()),
            highlighted_option_prefix: Styled::new(symbols::STEP).with_fg(self.inquire_info()),
            error_message: inquire::ui::ErrorMessageRenderConfig::default_colored()
                .with_prefix(Styled::new(symbols::ERROR).with_fg(self.inquire_error())),
            help_message: StyleSheet::new()
                .with_fg(self.inquire_dim())
                .with_attr(Attributes::ITALIC),
            answer: StyleSheet::new().with_fg(self.inquire_success()),
            ..RenderConfig::default_colored()
        }
    }

    /// Registers this theme globally for inquire prompts.
    pub fn register(&self) {
        inquire::set_global_render_config(self.render_config());
    }

    // --- Crossterm colors for printer ---

    /// Returns the success color (green, base0B).
    #[must_use]
    pub fn success(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base0b)
    }

    /// Returns the error color (red, base08).
    #[must_use]
    pub fn error(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base08)
    }

    /// Returns the warning color (yellow, base0A).
    #[must_use]
    pub fn warning(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base0a)
    }

    /// Returns the info color (cyan, base0C).
    #[must_use]
    pub fn info(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base0c)
    }

    /// Returns the primary/prompt color (blue, base0D).
    #[must_use]
    pub fn primary(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base0d)
    }

    /// Returns the dim/comment color (base03).
    #[must_use]
    pub fn dim(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base03)
    }

    /// Returns the foreground color (base05).
    #[must_use]
    pub fn foreground(&self) -> CrosstermColor {
        to_crossterm(&self.palette.base05)
    }

    // --- Inquire colors ---

    fn inquire_success(&self) -> InquireColor {
        to_inquire(&self.palette.base0b)
    }

    fn inquire_error(&self) -> InquireColor {
        to_inquire(&self.palette.base08)
    }

    fn inquire_info(&self) -> InquireColor {
        to_inquire(&self.palette.base0c)
    }

    fn inquire_primary(&self) -> InquireColor {
        to_inquire(&self.palette.base0d)
    }

    fn inquire_dim(&self) -> InquireColor {
        to_inquire(&self.palette.base03)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(flow_core::theme::builtin::flow())
    }
}

/// Converts a hex color to an inquire color.
fn to_inquire(hex: &HexColor) -> InquireColor {
    match hex.to_rgb() {
        Ok((r, g, b)) => InquireColor::Rgb { r, g, b },
        Err(_) => InquireColor::White,
    }
}

/// Converts a hex color to a crossterm color.
fn to_crossterm(hex: &HexColor) -> CrosstermColor {
    match hex.to_rgb() {
        Ok((r, g, b)) => CrosstermColor::Rgb { r, g, b },
        Err(_) => CrosstermColor::White,
    }
}

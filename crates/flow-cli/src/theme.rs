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

use crossterm::style::Color as CrosstermColor;
use flow_theme::{Base16Palette, HexColor, Theme};
use inquire::ui::{Attributes, Color as InquireColor, ErrorMessageRenderConfig, RenderConfig, StyleSheet, Styled};
use tabled::settings::Color as TabledColor;

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

/// A theme instance containing a base16 color palette with pre-computed colors.
///
/// Colors are converted from hex once at construction time and cached for
/// efficient access. The theme provides colors for both `inquire` prompts
/// and `crossterm` terminal output.
#[derive(Debug, Clone)]
pub struct CliTheme {
    palette: Base16Palette,
}

impl CliTheme {
    /// Creates a new theme from a base16 palette, pre-computing all colors.
    #[must_use]
    pub const fn new(palette: Base16Palette) -> Self {
        Self { palette }
    }

    /// Creates the inquire render configuration for this theme.
    #[must_use]
    pub fn render_config(&self) -> RenderConfig<'static> {
        let prompt_prefix = Styled::new(symbols::PROMPT).with_fg(self.primary().to_inquire());
        let answered_prompt_prefix = Styled::new(symbols::SUCCESS).with_fg(self.success().to_inquire());
        let prompt = StyleSheet::new()
            .with_fg(self.primary().to_inquire())
            .with_attr(Attributes::BOLD);
        let default_value = StyleSheet::new().with_fg(self.dim().to_inquire());
        let placeholder = StyleSheet::new()
            .with_fg(self.dim().to_inquire())
            .with_attr(Attributes::ITALIC);
        let help_message = StyleSheet::new()
            .with_fg(self.dim().to_inquire())
            .with_attr(Attributes::ITALIC);
        let text_input = StyleSheet::empty();
        let error_message = ErrorMessageRenderConfig::default_colored()
            .with_prefix(Styled::new(symbols::ERROR).with_fg(self.error().to_inquire()))
            .with_message(StyleSheet::new().with_fg(self.error().to_inquire()));
        let answer = StyleSheet::new().with_fg(self.foreground().to_inquire());
        let canceled_prompt_indicator = Styled::new("<canceled>").with_fg(self.error().to_inquire());
        let highlighted_option_prefix = Styled::new(symbols::STEP).with_fg(self.highlight().to_inquire());
        let selected_checkbox = Styled::new("[x]").with_fg(self.success().to_inquire());
        let unselected_checkbox = Styled::new("[ ]").with_fg(self.dim().to_inquire());
        let selected_option = Some(
            StyleSheet::new()
                .with_fg(self.palette.base09.to_inquire())
                .with_attr(Attributes::BOLD),
        );

        RenderConfig {
            prompt_prefix,
            answered_prompt_prefix,
            prompt,
            default_value,
            placeholder,
            help_message,
            text_input,
            error_message,
            answer,
            canceled_prompt_indicator,
            highlighted_option_prefix,
            selected_checkbox,
            unselected_checkbox,
            selected_option,
            ..RenderConfig::default_colored()
        }
    }

    /// Registers this theme globally for inquire prompts.
    pub fn register(&self) {
        inquire::set_global_render_config(self.render_config());
    }
}

impl Theme for CliTheme {
    fn palette(&self) -> &Base16Palette {
        &self.palette
    }
}

impl Default for CliTheme {
    fn default() -> Self {
        Self::new(Base16Palette::default())
    }
}

pub trait HexColorExt {
    /// Converts the hex color to an inquire color.
    fn to_inquire(&self) -> InquireColor;

    /// Converts the hex color to a crossterm color.
    fn to_crossterm(&self) -> CrosstermColor;

    /// Converts the hex color to a tabled color.
    fn to_tabled(&self) -> TabledColor;
}

impl HexColorExt for HexColor {
    fn to_inquire(&self) -> InquireColor {
        match self.to_rgb() {
            Ok((r, g, b)) => InquireColor::Rgb { r, g, b },
            Err(_) => InquireColor::White,
        }
    }

    fn to_crossterm(&self) -> CrosstermColor {
        match self.to_rgb() {
            Ok((r, g, b)) => CrosstermColor::Rgb { r, g, b },
            Err(_) => CrosstermColor::White,
        }
    }

    fn to_tabled(&self) -> TabledColor {
        match self.to_rgb() {
            Ok((r, g, b)) => TabledColor::rgb_fg(r, g, b),
            Err(_) => TabledColor::empty(),
        }
    }
}

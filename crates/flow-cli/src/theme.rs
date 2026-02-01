//! Inquire theme configuration for consistent CLI styling.
//!
//! This module provides a unified theme for `inquire` prompts that aligns
//! with the printer symbols and colors established throughout the CLI.

#![allow(dead_code)]

use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

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

/// Creates the Flow CLI render configuration for inquire prompts.
///
/// The theme uses:
/// - **Cyan**: Prompts, info, selections
/// - **Green**: Success, confirmed answers
/// - **Red**: Errors
/// - **Dim/Italic**: Help text, hints
#[must_use]
pub fn render_config() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new(symbols::PROMPT).with_fg(Color::LightCyan),
        answered_prompt_prefix: Styled::new(symbols::SUCCESS).with_fg(Color::LightGreen),
        highlighted_option_prefix: Styled::new(symbols::STEP).with_fg(Color::LightCyan),
        error_message: inquire::ui::ErrorMessageRenderConfig::default_colored()
            .with_prefix(Styled::new(symbols::ERROR).with_fg(Color::LightRed)),
        help_message: StyleSheet::new().with_attr(Attributes::ITALIC),
        answer: StyleSheet::new().with_fg(Color::LightGreen),
        ..RenderConfig::default_colored()
    }
}

/// Initializes the global inquire theme.
///
/// This should be called once at CLI startup, before any prompts are displayed.
pub fn init() {
    inquire::set_global_render_config(render_config());
}

//! Output formatting and printing for the CLI.
//!
//! This module provides the [`Printer`] struct, which handles all CLI output
//! with support for different output modes (normal, JSON, verbose, quiet).
//! Colors are derived from the provided theme's base16 palette.

#![allow(dead_code)]

use std::io::{stderr, stdout, Write};

use crossterm::style::Stylize;
use flow_theme::Theme as _;
use miette::{IntoDiagnostic, Result};
use tabled::{
    settings::{object::Rows, Color as TabledColor, Modify, Style},
    Table, Tabled,
};

use crate::theme::{symbols, CliTheme, HexColorExt};

/// Handles CLI output with support for multiple output modes.
///
/// The printer respects three flags that control output behavior:
///
/// - **JSON mode**: Suppresses human-readable output; only [`json`](Self::json) produces output
/// - **Verbose mode**: Enables [`verbose`](Self::verbose) and [`debug`](Self::debug) output
/// - **Quiet mode**: Suppresses all non-error output
///
/// Colors are determined by the provided theme's base16 palette.
#[allow(clippy::struct_excessive_bools)]
pub struct Printer {
    theme: CliTheme,
    json: bool,
    verbose: bool,
    trace: bool,
    quiet: bool,
}

impl Printer {
    /// Creates a new printer with the specified theme and output modes.
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub const fn new(theme: CliTheme, json: bool, verbose: bool, trace: bool, quiet: bool) -> Self {
        Self {
            theme,
            json,
            verbose: verbose || trace,
            trace,
            quiet,
        }
    }

    /// Returns a reference to the theme.
    #[must_use]
    pub const fn theme(&self) -> &CliTheme {
        &self.theme
    }

    /// Prints a plain message to stdout.
    pub fn print(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}", message.as_ref());
        }
    }

    /// Prints a success message with a green checkmark.
    pub fn success(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.success().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::SUCCESS.with(color),
                message.as_ref().with(color).bold()
            );
        }
    }

    /// Prints an informational message with a cyan info icon.
    pub fn info(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.info().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::INFO.with(color),
                message.as_ref().with(color)
            );
        }
    }

    /// Prints a warning message with a yellow warning icon.
    pub fn warning(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.warning().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::WARN.with(color),
                message.as_ref().with(color).bold()
            );
        }
    }

    /// Prints a step indicator with an arrow prefix.
    pub fn step(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.dim().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::STEP.with(self.theme.info().to_crossterm()),
                message.as_ref().with(color)
            );
        }
    }

    /// Prints a verbose message (only shown with `--verbose` flag).
    pub fn verbose(&self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let color = self.theme.dim().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::DEBUG.with(color),
                message.as_ref().with(color)
            );
        }
    }

    /// Prints a debug key-value pair (only shown with `--verbose` flag).
    pub fn debug(&self, label: impl AsRef<str>, value: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let color = self.theme.dim().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}: {}",
                symbols::DEBUG.with(color),
                label.as_ref().with(color),
                value.as_ref().with(color).italic()
            );
        }
    }

    /// Prints a trace message (only shown with `--trace` flag).
    pub fn trace(&self, message: impl AsRef<str>) {
        if self.trace && !self.quiet && !self.json {
            let color = self.theme.dim().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::DEBUG.with(color),
                message.as_ref().with(color)
            );
        }
    }

    /// Prints an error message to stderr with a red error icon.
    pub fn error(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.error().to_crossterm();
            let _ = writeln!(
                stderr(),
                "{} {}",
                symbols::ERROR.with(color),
                message.as_ref().with(color).bold()
            );
        }
    }

    /// Prints a section heading with a section symbol.
    pub fn heading(&self, heading: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let color = self.theme.primary().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::HEADING.with(color),
                heading.as_ref().with(color).bold().underlined()
            );
        }
    }

    /// Prints a key-value pair with indentation.
    pub fn kv(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "  {}: {}",
                key.as_ref().with(self.theme.info().to_crossterm()).bold(),
                value.as_ref().with(self.theme.foreground().to_crossterm())
            );
        }
    }

    /// Prints a table from an iterator of `Tabled` items.
    ///
    /// Table headers are styled with the theme's primary color and bold text.
    pub fn table<T, I>(&self, items: I)
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        if !self.quiet && !self.json {
            let header_color = self.theme.primary().to_tabled() | TabledColor::BOLD;
            let table = Table::new(items)
                .with(Style::rounded())
                .with(Modify::new(Rows::first()).with(header_color))
                .to_string();
            let _ = writeln!(stdout(), "{table}");
        }
    }

    /// Prints a blank line.
    pub fn blank(&self) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout());
        }
    }

    /// Outputs a value as pretty-printed JSON (only in JSON mode).
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn json<T: serde::Serialize>(&self, value: &T) -> Result<()> {
        if self.json {
            let json = serde_json::to_string_pretty(value).into_diagnostic()?;
            let _ = writeln!(stdout(), "{json}");
        }
        Ok(())
    }
}

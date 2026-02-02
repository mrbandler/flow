//! Output formatting and printing for the CLI.
//!
//! This module provides the [`Printer`] struct, which handles all CLI output
//! with support for different output modes (normal, JSON, verbose, quiet).
//! Colors are derived from the provided theme's base16 palette.

#![allow(dead_code)]

use std::io::{stderr, stdout, Write};

use crossterm::style::{Color, Stylize};
use miette::{IntoDiagnostic, Result};
use tabled::{settings::Style, Table, Tabled};

use crate::theme::{symbols, Theme};

/// Handles CLI output with support for multiple output modes.
///
/// The printer respects three flags that control output behavior:
///
/// - **JSON mode**: Suppresses human-readable output; only [`json`](Self::json) produces output
/// - **Verbose mode**: Enables [`verbose`](Self::verbose) and [`debug`](Self::debug) output
/// - **Quiet mode**: Suppresses all non-error output
///
/// Colors are determined by the provided theme's base16 palette.
pub struct Printer<'a> {
    theme: &'a Theme,
    json: bool,
    verbose: bool,
    quiet: bool,
}

impl<'a> Printer<'a> {
    /// Creates a new printer with the specified theme and output modes.
    #[must_use]
    pub const fn new(theme: &'a Theme, json: bool, verbose: bool, quiet: bool) -> Self {
        Self {
            theme,
            json,
            verbose,
            quiet,
        }
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
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::SUCCESS,
                message.as_ref().with(self.theme.success()).bold()
            );
        }
    }

    /// Prints an informational message with a cyan info icon.
    pub fn info(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::INFO,
                message.as_ref().with(self.theme.info())
            );
        }
    }

    /// Prints a warning message with a yellow warning icon.
    pub fn warning(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::WARN,
                message.as_ref().with(self.theme.warning()).bold()
            );
        }
    }

    /// Prints a step indicator with an arrow prefix.
    pub fn step(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::STEP,
                message.as_ref().with(self.theme.dim())
            );
        }
    }

    /// Prints a verbose message (only shown with `--verbose` flag).
    pub fn verbose(&self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::DEBUG,
                message.as_ref().with(self.theme.dim())
            );
        }
    }

    /// Prints a debug key-value pair (only shown with `--verbose` flag).
    pub fn debug(&self, label: impl AsRef<str>, value: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}: {}",
                symbols::DEBUG,
                label.as_ref().with(self.theme.dim()),
                value.as_ref().with(self.theme.dim()).italic()
            );
        }
    }

    /// Prints an error message to stderr with a red error icon.
    pub fn error(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stderr(),
                "{} {}",
                symbols::ERROR,
                message.as_ref().with(self.theme.error()).bold()
            );
        }
    }

    /// Prints a section heading with a section symbol.
    pub fn heading(&self, heading: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::HEADING,
                heading
                    .as_ref()
                    .with(self.theme.primary())
                    .bold()
                    .underlined()
            );
        }
    }

    /// Prints a key-value pair with indentation.
    pub fn kv(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "  {}: {}",
                key.as_ref().with(self.theme.info()).bold(),
                value.as_ref().with(Color::White)
            );
        }
    }

    /// Prints a table from an iterator of `Tabled` items.
    pub fn table<T, I>(&self, items: I)
    where
        T: Tabled,
        I: IntoIterator<Item = T>,
    {
        if !self.quiet && !self.json {
            let table = Table::new(items).with(Style::rounded()).to_string();
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

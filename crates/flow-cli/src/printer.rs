//! Output formatting and printing for the CLI.
//!
//! This module provides the [`Printer`] struct, which handles all CLI output
//! with support for different output modes (normal, JSON, verbose, quiet).

#![allow(dead_code)]

use std::io::{stderr, stdout, Write};

use crossterm::style::Stylize;
use miette::{IntoDiagnostic, Result};
use tabled::{settings::Style, Table, Tabled};

// Prefix symbols for different message types.
const SUCCESS: &str = "\u{2713} "; // ✓
const INFO: &str = "\u{2022} "; // •
const WARN: &str = "! ";
const ERROR: &str = "\u{2717} "; // ✗
const STEP: &str = "\u{2192} "; // →
const DEBUG: &str = "? ";
const HEADING: &str = "\u{00A7} "; // §

/// Handles CLI output with support for multiple output modes.
///
/// The printer respects three flags that control output behavior:
///
/// - **JSON mode**: Suppresses human-readable output; only [`json`](Self::json) produces output
/// - **Verbose mode**: Enables [`verbose`](Self::verbose) and [`debug`](Self::debug) output
/// - **Quiet mode**: Suppresses all non-error output
#[derive(Debug, Clone)]
pub struct Printer {
    json: bool,
    verbose: bool,
    quiet: bool,
}

impl Printer {
    /// Creates a new printer with the specified output modes.
    #[must_use]
    pub const fn new(json: bool, verbose: bool, quiet: bool) -> Self {
        Self { json, verbose, quiet }
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
            let _ = writeln!(stdout(), "{}{}", SUCCESS, message.as_ref().green().bold());
        }
    }

    /// Prints an informational message with a blue info icon.
    pub fn info(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}{}", INFO, message.as_ref().cyan());
        }
    }

    /// Prints a warning message with a yellow warning icon.
    pub fn warning(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}{}", WARN, message.as_ref().yellow().bold());
        }
    }

    /// Prints a step indicator with an arrow prefix.
    pub fn step(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}{}", STEP, message.as_ref().dim());
        }
    }

    /// Prints a verbose message (only shown with `--verbose` flag).
    pub fn verbose(&self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}{}", DEBUG, message.as_ref().dim());
        }
    }

    /// Prints a debug key-value pair (only shown with `--verbose` flag).
    pub fn debug(&self, label: impl AsRef<str>, value: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = writeln!(
                stdout(),
                "{}{}: {}",
                DEBUG,
                label.as_ref().dim(),
                value.as_ref().dim().italic()
            );
        }
    }

    /// Prints an error message to stderr with a red error icon.
    pub fn error(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stderr(), "{}{}", ERROR, message.as_ref().red().bold());
        }
    }

    /// Prints a section heading with a section symbol.
    pub fn heading(&self, heading: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "{}{}", HEADING, heading.as_ref().bold().underlined());
        }
    }

    /// Prints a key-value pair with indentation.
    pub fn kv(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = writeln!(stdout(), "  {}: {}", key.as_ref().cyan().bold(), value.as_ref().white());
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

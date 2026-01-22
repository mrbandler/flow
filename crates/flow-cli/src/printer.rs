//! Output formatting and printing for the CLI.
//!
//! This module provides the [`Printer`] struct, which handles all CLI output
//! with support for different output modes (normal, JSON, verbose, quiet).

use console::{style, Emoji, Term};
use miette::{IntoDiagnostic, Result};

// Emojis with fallbacks for terminals that don't support them.
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "* ");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "[i] ");
static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "[+] ");
static WARN: Emoji<'_, '_> = Emoji("⚠️  ", "[!] ");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", "[x] ");
static DEBUG: Emoji<'_, '_> = Emoji("🔍 ", "[?] ");
static ARROW: Emoji<'_, '_> = Emoji("→ ", "-> ");

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

    /// Returns `true` if JSON output mode is enabled.
    #[must_use]
    pub const fn is_json(&self) -> bool {
        self.json
    }

    /// Prints a plain message to stdout.
    pub fn print(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(message.as_ref());
        }
    }

    /// Prints a success message with a green checkmark.
    pub fn success(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", SUCCESS, style(message.as_ref()).green().bold()));
        }
    }

    /// Prints an informational message with a blue info icon.
    pub fn info(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", INFO, style(message.as_ref()).cyan()));
        }
    }

    /// Prints a warning message with a yellow warning icon.
    pub fn warning(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", WARN, style(message.as_ref()).yellow().bold()));
        }
    }

    /// Prints a step indicator with an arrow prefix.
    pub fn step(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", ARROW, style(message.as_ref()).dim()));
        }
    }

    /// Prints a verbose message (only shown with `--verbose` flag).
    pub fn verbose(&self, message: impl AsRef<str>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", DEBUG, style(message.as_ref()).dim()));
        }
    }

    /// Prints a debug key-value pair (only shown with `--verbose` flag).
    pub fn debug(&self, label: impl Into<String>, value: impl Into<String>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!(
                "{}{}: {}",
                DEBUG,
                style(label.into()).dim(),
                style(value.into()).dim().italic()
            ));
        }
    }

    /// Prints an error message to stderr with a red error icon.
    pub fn error(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _ = Term::stderr().write_line(&format!("{}{}", ERROR, style(message.as_ref()).red().bold()));
        }
    }

    /// Prints a section heading with a sparkle icon.
    pub fn heading(&self, heading: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", SPARKLE, style(heading.into()).bold().underlined()));
        }
    }

    /// Prints a key-value pair with indentation.
    pub fn kv(&self, key: impl Into<String>, value: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!(
                "  {}: {}",
                style(key.into()).cyan().bold(),
                style(value.into()).white()
            ));
        }
    }

    /// Prints a blank line.
    pub fn blank(&self) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line("");
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
            let _ = Term::stdout().write_line(&json);
        }
        Ok(())
    }
}

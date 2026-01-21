use console::{style, Emoji, Term};
use miette::{IntoDiagnostic, Result};

// Emojis with fallbacks for terminals that don't support them
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "* ");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "[i] ");
static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "[+] ");
static WARN: Emoji<'_, '_> = Emoji("⚠️  ", "[!] ");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", "[x] ");
static DEBUG: Emoji<'_, '_> = Emoji("🔍 ", "[?] ");
static ARROW: Emoji<'_, '_> = Emoji("→ ", "-> ");

/// Handles all CLI output printing with support for JSON, verbose, and quiet modes.
#[derive(Debug, Clone)]
pub struct Printer {
    json: bool,
    verbose: bool,
    quiet: bool,
}

impl Printer {
    #[must_use]
    pub const fn new(json: bool, verbose: bool, quiet: bool) -> Self {
        Self { json, verbose, quiet }
    }

    #[must_use]
    pub const fn is_json(&self) -> bool {
        self.json
    }

    pub fn print(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(message.into().as_str());
        }
    }

    pub fn success(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", SUCCESS, style(message.into()).green().bold()));
        }
    }

    pub fn info(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", INFO, style(message.into()).cyan()));
        }
    }

    pub fn warning(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", WARN, style(message.into()).yellow().bold()));
        }
    }

    pub fn step(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", ARROW, style(message.into()).dim()));
        }
    }

    pub fn verbose(&self, message: impl Into<String>) {
        if self.verbose && !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", DEBUG, style(message.into()).dim()));
        }
    }

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

    pub fn error(&self, message: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stderr().write_line(&format!("{}{}", ERROR, style(message.into()).red().bold()));
        }
    }

    pub fn heading(&self, heading: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!("{}{}", SPARKLE, style(heading.into()).bold().underlined()));
        }
    }

    pub fn kv(&self, key: impl Into<String>, value: impl Into<String>) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line(&format!(
                "  {}: {}",
                style(key.into()).cyan().bold(),
                style(value.into()).white()
            ));
        }
    }

    pub fn blank(&self) {
        if !self.quiet && !self.json {
            let _ = Term::stdout().write_line("");
        }
    }

    pub fn json<T: serde::Serialize>(&self, value: &T) -> Result<()> {
        if self.json {
            let json = serde_json::to_string_pretty(value).into_diagnostic()?;
            let _ = Term::stdout().write_line(&json);
        }
        Ok(())
    }
}

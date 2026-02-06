//! Output formatting and printing for the CLI.
//!
//! This module provides the [`Printer`] struct, which handles all CLI output
//! with support for different output modes (normal, JSON, verbose, quiet).
//! Colors are derived from the provided theme's base16 palette.
//!
//! It also provides [`Spinner`], a single-line animated spinner for
//! long-running operations. The spinner always stays at the bottom of the
//! output — when other output arrives (including trace logs), the spinner
//! line is cleared, the new output is written, and the spinner re-renders
//! on its next tick.

#![allow(dead_code)]

use std::io::{stderr, stdout, Write};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crossterm::{cursor, style::Stylize, terminal};
use flow_theme::Theme as _;
use miette::{IntoDiagnostic, Result};
use tabled::{
    settings::{object::Rows, Color as TabledColor, Modify, Style},
    Table, Tabled,
};
use tokio::task::JoinHandle;

use crate::theme::{symbols, CliTheme, HexColorExt};

// --- Spinner internals ---

/// Animation frames (braille dot pattern).
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Interval between animation frames.
const SPINNER_TICK_MS: u64 = 80;

/// Shared spinner line state. `None` means no spinner is active.
type SpinnerState = Arc<Mutex<Option<SpinnerLine>>>;

struct SpinnerLine {
    message: String,
    frame_idx: usize,
}

impl SpinnerLine {
    /// Renders the current frame to the terminal.
    fn render(&self, theme: &CliTheme) {
        let frame = SPINNER_FRAMES[self.frame_idx % SPINNER_FRAMES.len()];
        let color = theme.info().to_crossterm();

        Printer::clear_line();
        let mut out = stdout();
        let _ = write!(out, "{} {}", frame.with(color), self.message.as_str().with(color));
        let _ = out.flush();
    }
}

// --- Printer ---

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
    spinner: SpinnerState,
}

impl Printer {
    /// Creates a new printer with the specified theme and output modes.
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn new(theme: CliTheme, json: bool, verbose: bool, trace: bool, quiet: bool) -> Self {
        Self {
            theme,
            json,
            verbose: verbose || trace,
            trace,
            quiet,
            spinner: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns a reference to the theme.
    #[must_use]
    pub const fn theme(&self) -> &CliTheme {
        &self.theme
    }

    /// Creates a themed terminal spinner for long-running operations.
    ///
    /// In quiet or JSON mode, returns a no-op spinner that silently
    /// discards all operations.
    pub fn spinner(&self, message: impl AsRef<str>) -> Spinner {
        if self.quiet || self.json {
            return Spinner::noop();
        }
        Spinner::start(message, self.theme.clone(), Arc::clone(&self.spinner))
    }

    /// Prints a plain message to stdout.
    pub fn print(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _guard = self.suspend_spinner();
            let _ = writeln!(stdout(), "{}", message.as_ref());
        }
    }

    /// Prints a success message with a green checkmark.
    pub fn success(&self, message: impl AsRef<str>) {
        if !self.quiet && !self.json {
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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
            let _guard = self.suspend_spinner();
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

    /// Clears the spinner line if active, returning the held lock so output
    /// can be written without the spinner tick interleaving.
    fn suspend_spinner(&self) -> MutexGuard<'_, Option<SpinnerLine>> {
        let guard = self.spinner.lock().unwrap_or_else(PoisonError::into_inner);
        if guard.is_some() {
            Self::clear_line();
        }
        guard
    }

    /// Clears the current terminal line.
    fn clear_line() {
        let mut out = stdout();
        let _ = crossterm::execute!(
            out,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine)
        );
    }
}

// --- Spinner ---

/// A single-line terminal spinner for long-running operations.
///
/// Created via [`Printer::spinner`]. In quiet or JSON mode it is a no-op.
/// The spinner cleans up automatically when dropped.
pub struct Spinner {
    inner: Option<SpinnerTask>,
}

struct SpinnerTask {
    state: SpinnerState,
    theme: CliTheme,
    handle: JoinHandle<()>,
}

impl Spinner {
    /// Creates and starts a spinner with the given message.
    fn start(message: impl AsRef<str>, theme: CliTheme, state: SpinnerState) -> Self {
        {
            let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
            *guard = Some(SpinnerLine {
                message: message.as_ref().to_string(),
                frame_idx: 0,
            });
        }

        let tick_state = Arc::clone(&state);
        let tick_theme = theme.clone();
        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(SPINNER_TICK_MS)).await;

                let mut guard = tick_state.lock().unwrap_or_else(PoisonError::into_inner);
                if let Some(line) = guard.as_mut() {
                    line.render(&tick_theme);
                    line.frame_idx = line.frame_idx.wrapping_add(1);
                } else {
                    break;
                }
            }
        });

        Self {
            inner: Some(SpinnerTask { state, theme, handle }),
        }
    }

    /// Creates a no-op spinner that silently discards all operations.
    const fn noop() -> Self {
        Self { inner: None }
    }

    /// Updates the spinner's display text.
    pub fn set_message(&self, message: impl AsRef<str>) {
        if let Some(task) = &self.inner {
            let mut guard = task.state.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(line) = guard.as_mut() {
                line.message = message.as_ref().to_string();
            }
        }
    }

    /// Stops the spinner and prints a themed success message.
    pub fn success(mut self, message: impl AsRef<str>) {
        if let Some(task) = self.inner.take() {
            task.handle.abort();
            task.state
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();

            Printer::clear_line();
            let color = task.theme.success().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::SUCCESS.with(color),
                message.as_ref().with(color).bold(),
            );
        }
    }

    /// Stops the spinner and prints a themed error message.
    pub fn error(mut self, message: impl AsRef<str>) {
        if let Some(task) = self.inner.take() {
            task.handle.abort();
            task.state
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();

            Printer::clear_line();
            let color = task.theme.error().to_crossterm();
            let _ = writeln!(
                stdout(),
                "{} {}",
                symbols::ERROR.with(color),
                message.as_ref().with(color).bold(),
            );
        }
    }

    /// Stops the spinner silently without printing a final message.
    pub fn stop(mut self) {
        if let Some(task) = self.inner.take() {
            Self::teardown(&task);
        }
    }

    fn teardown(task: &SpinnerTask) {
        task.handle.abort();
        task.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        Printer::clear_line();
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if let Some(task) = self.inner.take() {
            Self::teardown(&task);
        }
    }
}

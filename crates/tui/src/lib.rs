//! Flow Terminal User Interface
//!
//! This crate provides a terminal-based user interface for Flow using the
//! [ratatui](https://ratatui.rs) library.
//!
//! # Features
//!
//! - Full-screen terminal interface for browsing and editing notes
//! - Keyboard-driven navigation
//! - Vim-inspired keybindings
//! - Real-time search and filtering
//! - Graph visualization in the terminal
//!
//! # Architecture
//!
//! The TUI is built using the Elm architecture pattern:
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │                  App                    │
//! │  ┌─────────┐  ┌──────────┐  ┌────────┐  │
//! │  │  Model  │──│  Update  │──│  View  │  │
//! │  └─────────┘  └──────────┘  └────────┘  │
//! └─────────────────────────────────────────┘
//! ```
//!
//! - **Model**: Application state (`TuiApp`)
//! - **Update**: Event handling and state transitions
//! - **View**: Rendering the UI based on current state
//!
//! # Example
//!
//! ```rust,ignore
//! use flow_tui::run;
//!
//! fn main() -> miette::Result<()> {
//!     run()
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/flow-tui/0.1.0")]

use std::io::{self, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use flow_app::App;
use miette::{Context, IntoDiagnostic, Result};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

/// Terminal type alias for the ratatui backend
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// The main TUI application state
#[derive(Debug)]
pub struct TuiApp {
    /// The underlying Flow application
    app: App,
    /// Whether the application should exit
    should_quit: bool,
}

impl TuiApp {
    /// Creates a new TUI application instance
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying App cannot be initialized.
    pub fn new() -> Result<Self> {
        let app = App::new()?;
        Ok(Self {
            app,
            should_quit: false,
        })
    }

    /// Returns whether the application should quit
    #[must_use]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Handles keyboard input events
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            },
            // TODO: Add more keybindings
            _ => {},
        }
    }

    /// Renders the UI to the terminal
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    pub fn render(&self, frame: &mut Frame<'_>) {
        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new("Flow - Note taking for developers")
            .style(Style::default().fg(Color::Cyan).bold())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(header, chunks[0]);

        // Main content area
        let content = Paragraph::new(self.content_text())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .title(" Notes ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        frame.render_widget(content, chunks[1]);

        // Footer with help text
        let footer = Paragraph::new("Press 'q' or Esc to quit")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(footer, chunks[2]);
    }

    /// Returns the content text to display
    fn content_text(&self) -> String {
        if self.app.has_active_graph() {
            if let Some(graph) = self.app.active_graph() {
                return format!(
                    "Active graph: {}\nPath: {}",
                    graph.name(),
                    graph.path().display()
                );
            }
        }

        String::from(
            "No active graph.\n\n\
             Use the CLI to initialize or open a graph:\n\n\
             • flow init <path>  - Initialize a new graph\n\
             • flow open <name>  - Open an existing graph\n\n\
             Then run 'flow tui' or 'flow-tui' to launch the TUI.",
        )
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self {
            app: App::default(),
            should_quit: false,
        }
    }
}

/// Initialize the terminal for TUI rendering
///
/// # Errors
///
/// Returns an error if terminal initialization fails.
fn init_terminal() -> Result<Terminal> {
    enable_raw_mode()
        .into_diagnostic()
        .context("Failed to enable raw mode")?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .into_diagnostic()
        .context("Failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)
        .into_diagnostic()
        .context("Failed to create terminal")?;

    Ok(terminal)
}

/// Restore the terminal to its original state
///
/// # Errors
///
/// Returns an error if terminal restoration fails.
fn restore_terminal(terminal: &mut Terminal) -> Result<()> {
    disable_raw_mode()
        .into_diagnostic()
        .context("Failed to disable raw mode")?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .into_diagnostic()
        .context("Failed to leave alternate screen")?;

    terminal
        .show_cursor()
        .into_diagnostic()
        .context("Failed to show cursor")?;

    Ok(())
}

/// Runs the TUI application
///
/// This is the main entry point for the terminal user interface.
/// It initializes the terminal, runs the main event loop, and
/// restores the terminal on exit.
///
/// # Errors
///
/// Returns an error if:
/// - Terminal initialization fails
/// - The TUI application cannot be created
/// - An I/O error occurs during the event loop
/// - Terminal restoration fails
///
/// # Example
///
/// ```rust,ignore
/// fn main() -> miette::Result<()> {
///     flow_tui::run()
/// }
/// ```
pub fn run() -> Result<()> {
    // Initialize terminal
    let mut terminal = init_terminal()?;

    // Create app state
    let mut app = TuiApp::new()?;

    // Main event loop
    let result = run_event_loop(&mut terminal, &mut app);

    // Restore terminal (always, even on error)
    restore_terminal(&mut terminal)?;

    result
}

/// The main event loop
fn run_event_loop(terminal: &mut Terminal, app: &mut TuiApp) -> Result<()> {
    loop {
        // Draw the UI
        terminal
            .draw(|frame| app.render(frame))
            .into_diagnostic()
            .context("Failed to draw frame")?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))
            .into_diagnostic()
            .context("Failed to poll for events")?
        {
            if let Event::Key(key) = event::read()
                .into_diagnostic()
                .context("Failed to read event")?
            {
                // Only handle key press events, not release
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }

        // Check if we should quit
        if app.should_quit() {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_app_default_should_not_quit() {
        let app = TuiApp::default();
        assert!(!app.should_quit());
    }

    #[test]
    fn test_tui_app_quit_on_q() {
        let mut app = TuiApp::default();
        app.handle_key(KeyCode::Char('q'));
        assert!(app.should_quit());
    }

    #[test]
    fn test_tui_app_quit_on_esc() {
        let mut app = TuiApp::default();
        app.handle_key(KeyCode::Esc);
        assert!(app.should_quit());
    }

    #[test]
    fn test_tui_app_no_quit_on_other_keys() {
        let mut app = TuiApp::default();
        app.handle_key(KeyCode::Char('a'));
        assert!(!app.should_quit());
    }
}

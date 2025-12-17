//! Flow Desktop Application
//!
//! This crate provides a native desktop GUI for Flow using the
//! [iced](https://iced.rs) library.
//!
//! # Features
//!
//! - Native look and feel on all platforms
//! - Rich text editing with markdown support
//! - Knowledge graph visualization
//! - Drag and drop support
//! - System tray integration (planned)
//!
//! # Architecture
//!
//! The desktop app follows the Elm architecture pattern provided by iced:
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              Desktop App                │
//! │  ┌─────────┐  ┌──────────┐  ┌────────┐  │
//! │  │  State  │──│  Update  │──│  View  │  │
//! │  └─────────┘  └──────────┘  └────────┘  │
//! └─────────────────────────────────────────┘
//! ```
//!
//! - **State**: Application state (`FlowDesktop`)
//! - **Update**: Message handling and state transitions
//! - **View**: Widget tree construction
//!
//! # Example
//!
//! ```rust,ignore
//! use flow_desktop::run;
//!
//! fn main() -> miette::Result<()> {
//!     run()
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/flow-desktop/0.1.0")]

use flow_app::App;
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Center, Element, Fill, Task, Theme};
use miette::{Context, Result};

/// Messages that can be sent in the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Open a graph by name
    OpenGraph(String),
    /// Close the current graph
    CloseGraph,
    /// Quit the application
    Quit,
    /// No operation (used for unhandled events)
    Noop,
}

/// The main desktop application state
#[derive(Debug)]
pub struct FlowDesktop {
    /// The underlying Flow application
    app: App,
    /// Status message to display
    status: String,
}

impl FlowDesktop {
    /// Creates a new desktop application instance
    fn new() -> Self {
        let app = App::default();
        Self {
            app,
            status: String::from("Welcome to Flow"),
        }
    }

    /// Returns the window title
    fn title(&self) -> String {
        if let Some(graph) = self.app.active_graph() {
            format!("Flow - {}", graph.name())
        } else {
            String::from("Flow")
        }
    }

    /// Handles messages and updates application state
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenGraph(name_or_path) => {
                match self.app.open_graph(&name_or_path) {
                    Ok(graph) => {
                        self.status = format!("Opened graph: {}", graph.name());
                    },
                    Err(e) => {
                        self.status = format!("Failed to open graph: {e}");
                    },
                }
                Task::none()
            },
            Message::CloseGraph => {
                self.app.close_graph();
                self.status = String::from("Graph closed");
                Task::none()
            },
            Message::Quit => iced::exit(),
            Message::Noop => Task::none(),
        }
    }

    /// Renders the application view
    fn view(&self) -> Element<'_, Message> {
        // Header
        let header = container(
            row![
                text("Flow").size(24),
                horizontal_space(),
                button("Quit").on_press(Message::Quit),
            ]
            .spacing(10)
            .padding(10),
        )
        .style(container::bordered_box);

        // Main content
        let content = if let Some(graph) = self.app.active_graph() {
            container(
                column![
                    text(format!("Graph: {}", graph.name())).size(20),
                    text(format!("Path: {}", graph.path().display())).size(14),
                    button("Close Graph").on_press(Message::CloseGraph),
                ]
                .spacing(10),
            )
        } else {
            container(
                column![
                    text("No Active Graph").size(20),
                    text("Use the CLI to initialize or open a graph:").size(14),
                    text("• flow init <path>  - Initialize a new graph").size(12),
                    text("• flow open <name>  - Open an existing graph").size(12),
                ]
                .spacing(10),
            )
        };

        // Status bar
        let status_bar = container(text(&self.status).size(12)).padding(5);

        // Main layout
        container(
            column![header, content.height(Fill).center_x(Fill), status_bar,]
                .width(Fill)
                .height(Fill),
        )
        .center(Fill)
        .into()
    }

    /// Returns the theme for the application
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Default for FlowDesktop {
    fn default() -> Self {
        Self::new()
    }
}

/// Runs the desktop application
///
/// This is the main entry point for the desktop GUI.
/// It initializes the iced application and runs the event loop.
///
/// # Errors
///
/// Returns an error if:
/// - The GUI framework fails to initialize
/// - Window creation fails
///
/// # Example
///
/// ```rust,ignore
/// fn main() -> miette::Result<()> {
///     flow_desktop::run()
/// }
/// ```
pub fn run() -> Result<()> {
    iced::application(FlowDesktop::title, FlowDesktop::update, FlowDesktop::view)
        .theme(FlowDesktop::theme)
        .centered()
        .run_with(|| (FlowDesktop::new(), Task::none()))
        .map_err(|e| miette::miette!("Failed to run desktop application: {}", e))
        .context("Desktop application error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_app_default_title() {
        let app = FlowDesktop::default();
        assert_eq!(app.title(), "Flow");
    }

    #[test]
    fn test_desktop_app_default_status() {
        let app = FlowDesktop::default();
        assert_eq!(app.status, "Welcome to Flow");
    }

    #[test]
    fn test_desktop_app_close_graph_updates_status() {
        let mut app = FlowDesktop::default();
        let _ = app.update(Message::CloseGraph);
        assert_eq!(app.status, "Graph closed");
    }
}

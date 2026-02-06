//! Tracing integration for routing library logs through the CLI [`Printer`].
//!
//! This module provides a custom [`tracing`] layer that captures log events
//! from library crates (`flow-core`, `flow-app`, etc.) and routes them
//! through the [`Printer`] for centralized output formatting.
//!
//! Tracing levels are mapped to printer methods:
//!
//! | Tracing level       | Printer method        |
//! |---------------------|-----------------------|
//! | `error`             | [`Printer::error`]    |
//! | `warn`              | [`Printer::warning`]  |
//! | `info`              | [`Printer::verbose`]  |
//! | `debug`/`trace`     | [`Printer::trace`]    |

use std::fmt::Write as _;
use std::sync::Arc;

use tracing::field::{Field, Visit};
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

use crate::printer::Printer;

/// A tracing layer that routes log events through the CLI [`Printer`].
pub struct PrinterLayer {
    printer: Arc<Printer>,
}

impl PrinterLayer {
    /// Creates a new `PrinterLayer` with the given printer.
    pub const fn new(printer: Arc<Printer>) -> Self {
        Self { printer }
    }
}

impl<S: Subscriber> Layer<S> for PrinterLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);

        let message = visitor.message;
        if message.is_empty() {
            return;
        }

        match *event.metadata().level() {
            tracing::Level::ERROR => self.printer.error(&message),
            tracing::Level::WARN => self.printer.warning(&message),
            tracing::Level::INFO => self.printer.verbose(&message),
            _ => self.printer.trace(&message),
        }
    }
}

/// Visitor that extracts the message field from a tracing event.
#[derive(Default)]
struct MessageVisitor {
    message: String,
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message.clear();
            let _ = write!(self.message, "{value:?}");
        }
    }
}

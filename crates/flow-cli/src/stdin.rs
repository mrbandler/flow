//! Piped stdin support for pre-filling missing command arguments.
//!
//! When stdin is piped (not a TTY), this module reads all lines eagerly
//! and provides them as an iterator for commands to consume positionally.

use std::io::{self, BufRead};

use crossterm::tty::IsTty;

/// Lines read from piped stdin.
///
/// When stdin is piped, all lines are read eagerly on construction.
/// Commands consume lines positionally via the iterator returned by
/// [`into_iter`](Self::into_iter).
pub struct Stdin {
    lines: Vec<String>,
    piped: bool,
}

impl Stdin {
    /// Reads stdin if it is piped (not a TTY).
    ///
    /// If stdin is a TTY (interactive), returns an empty `StdinLines`
    /// with `is_piped() == false`.
    pub fn read() -> Self {
        let stdin = io::stdin();
        if stdin.is_tty() {
            return Self {
                lines: Vec::new(),
                piped: false,
            };
        }

        let lines = stdin
            .lock()
            .lines()
            .map_while(Result::ok)
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        Self { lines, piped: true }
    }

    /// Returns whether stdin was piped.
    pub const fn is_piped(&self) -> bool {
        self.piped
    }
}

impl IntoIterator for Stdin {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

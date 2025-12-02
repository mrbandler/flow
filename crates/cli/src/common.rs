//! Common types and utilities shared across all CLI commands.

use clap::Args;
use console::{style, Emoji, Term};
use flow_core::config::Config;
use flow_core::graph::Graph;
use miette::{Context, IntoDiagnostic, Result};
use std::path::{Path, PathBuf};

use crate::error::CliError;

// Emojis with fallbacks for terminals that don't support them
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "* ");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "[i] ");
static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "[+] ");
static WARN: Emoji<'_, '_> = Emoji("⚠️  ", "[!] ");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", "[x] ");
static DEBUG: Emoji<'_, '_> = Emoji("🔍 ", "[?] ");
static ARROW: Emoji<'_, '_> = Emoji("→ ", "-> ");

/// Converts a canonicalized path to a clean display string.
///
/// On Windows, canonicalized paths include the `\\?\` prefix which looks
/// odd in output. This function strips that prefix for cleaner display.
///
/// # Arguments
///
/// * `path` - The path to convert to a display string
///
/// # Returns
///
/// A clean string representation of the path without Windows extended-length prefix
pub fn path_to_display_string(path: &Path) -> String {
    let path_str = path.display().to_string();

    #[cfg(windows)]
    {
        // Remove Windows extended-length path prefix
        if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
            return stripped.to_string();
        }
    }

    path_str
}

/// Global flags available for all commands.
///
/// These flags are flattened into each command's args struct using `#[command(flatten)]`.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Args)]
/// pub struct MyCommandArgs {
///     #[command(flatten)]
///     pub global: GlobalArgs,
///
///     pub my_arg: String,
/// }
/// ```
#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Target specific graph by name or path (overrides active graph)
    #[arg(long, global = true)]
    pub graph: Option<String>,

    /// Detailed logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

impl GlobalArgs {
    /// Get the terminal for output
    fn term(&self) -> Term {
        Term::stdout()
    }

    /// Get the terminal for error output
    fn term_err(&self) -> Term {
        Term::stderr()
    }

    /// Load the target graph based on global flags and config.
    ///
    /// This method respects the `--graph` flag if provided (which can be either
    /// a registered graph name or a path), otherwise falls back to the active
    /// graph from the config.
    ///
    /// # Returns
    ///
    /// * `Result<Graph>` - The loaded graph
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The specified graph name is not registered
    /// - The specified graph path doesn't exist
    /// - No graph is specified and no active graph is set
    /// - The graph fails to load
    pub fn load_graph(&self) -> Result<Graph> {
        let config = Config::load()?;

        if let Some(ref name_or_path) = self.graph {
            if let Some(graph_config) = config.get_space_config(name_or_path) {
                Graph::load(&graph_config.path).with_context(|| {
                    format!(
                        "Failed to load graph from '{}'",
                        graph_config.path.display()
                    )
                })
            } else {
                let path = PathBuf::from(name_or_path);
                if !path.exists() {
                    return Err(CliError::graph_not_found(name_or_path).into());
                }
                Graph::load(&path).map_err(|_| CliError::invalid_graph(path.clone()).into())
            }
        } else {
            let active = config
                .get_active_space()
                .ok_or_else(|| CliError::NoActiveGraph)?;
            Graph::load(&active.path).with_context(|| {
                format!(
                    "Failed to load active graph from '{}'",
                    active.path.display()
                )
            })
        }
    }

    /// Print a message respecting the --quiet flag.
    ///
    /// When --json flag is set, this method does nothing as output
    /// should be handled via `print_json()`.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print
    pub fn print(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ = self.term().write_line(message);
        }
    }

    /// Print a success message with green color and icon.
    ///
    /// # Arguments
    ///
    /// * `message` - The success message to print
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.success("Graph initialized successfully");
    /// // Outputs: ✅ Graph initialized successfully (in green)
    /// ```
    pub fn success(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ =
                self.term()
                    .write_line(&format!("{}{}", SUCCESS, style(message).green().bold()));
        }
    }

    /// Print an info message with cyan color and icon.
    ///
    /// # Arguments
    ///
    /// * `message` - The info message to print
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.info("Loading graph configuration");
    /// // Outputs: ℹ️  Loading graph configuration (in cyan)
    /// ```
    pub fn info(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ = self
                .term()
                .write_line(&format!("{}{}", INFO, style(message).cyan()));
        }
    }

    /// Print a warning message with yellow color and icon.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning message to print
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.warning("Template support not yet implemented");
    /// // Outputs: ⚠️  Template support not yet implemented (in yellow)
    /// ```
    pub fn warning(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ = self
                .term()
                .write_line(&format!("{}{}", WARN, style(message).yellow().bold()));
        }
    }

    /// Print a step message with arrow icon (for showing progress).
    ///
    /// # Arguments
    ///
    /// * `message` - The step message to print
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.step("Registering graph in configuration");
    /// // Outputs: → Registering graph in configuration (in dim white)
    /// ```
    pub fn step(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ = self
                .term()
                .write_line(&format!("{}{}", ARROW, style(message).dim()));
        }
    }

    /// Print a verbose/debug message (only shown with --verbose flag).
    ///
    /// When --json flag is set, this method does nothing as output
    /// should be handled via `print_json()`.
    ///
    /// # Arguments
    ///
    /// * `message` - The verbose message to print
    pub fn print_verbose(&self, message: &str) {
        if self.verbose && !self.quiet && !self.json {
            let _ = self
                .term()
                .write_line(&format!("{}{}", DEBUG, style(message).dim()));
        }
    }

    /// Print a debug message with details (only shown with --verbose flag).
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the debug message
    /// * `value` - The value to display
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.debug("Graph path", &path.display().to_string());
    /// // Outputs: 🔍 Graph path: /path/to/graph (in dim, when --verbose)
    /// ```
    pub fn debug(&self, label: &str, value: &str) {
        if self.verbose && !self.quiet && !self.json {
            let _ = self.term().write_line(&format!(
                "{}{}: {}",
                DEBUG,
                style(label).dim(),
                style(value).dim().italic()
            ));
        }
    }

    /// Print an error message (always shown unless --quiet).
    ///
    /// When --json flag is set, this method does nothing as errors
    /// should be handled through the Result error chain.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to print
    pub fn print_error(&self, message: &str) {
        if !self.quiet && !self.json {
            let _ =
                self.term_err()
                    .write_line(&format!("{}{}", ERROR, style(message).red().bold()));
        }
    }

    /// Print a styled heading/section.
    ///
    /// # Arguments
    ///
    /// * `heading` - The heading text
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.heading("Cleaning Graphs");
    /// // Outputs: ✨ Cleaning Graphs (in bold)
    /// ```
    pub fn heading(&self, heading: &str) {
        if !self.quiet && !self.json {
            let _ = self.term().write_line(&format!(
                "{}{}",
                SPARKLE,
                style(heading).bold().underlined()
            ));
        }
    }

    /// Print a key-value pair (useful for showing results).
    ///
    /// # Arguments
    ///
    /// * `key` - The key/label
    /// * `value` - The value to display
    ///
    /// # Example
    ///
    /// ```ignore
    /// global.kv("Name", "my-notes");
    /// global.kv("Path", "/path/to/notes");
    /// // Outputs:
    /// //   Name: my-notes
    /// //   Path: /path/to/notes
    /// ```
    pub fn kv(&self, key: &str, value: &str) {
        if !self.quiet && !self.json {
            let _ = self.term().write_line(&format!(
                "  {}: {}",
                style(key).cyan().bold(),
                style(value).white()
            ));
        }
    }

    /// Print a blank line (for spacing).
    pub fn blank(&self) {
        if !self.quiet && !self.json {
            let _ = self.term().write_line("");
        }
    }

    /// Print JSON output (only when --json flag is set).
    ///
    /// This method serializes the given value to pretty-printed JSON
    /// and prints it to stdout. Only outputs when the --json flag is set.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to serialize and print as JSON
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or serialization error
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails
    pub fn print_json<T: serde::Serialize>(&self, value: &T) -> Result<()> {
        if self.json {
            let json = serde_json::to_string_pretty(value).into_diagnostic()?;
            let _ = self.term().write_line(&json);
        }
        Ok(())
    }
}

/// Trait for CLI commands with separated concerns.
///
/// This trait separates interactive mode, execution logic, and output formatting
/// for cleaner command implementations.
///
/// # Execution Flow
///
/// 1. `from_args` - Create command from parsed CLI arguments
/// 2. `interactive` - Collect missing arguments interactively (if not in JSON mode)
/// 3. `run` - Execute the command logic and return structured output
/// 4. `execute` - Orchestrate the above and handle output formatting
///
/// # Example
///
/// ```rust,ignore
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct InitOutput {
///     name: String,
///     path: String,
/// }
///
/// pub struct InitCommand {
///     args: InitArgs,
/// }
///
/// impl Command for InitCommand {
///     type Args = InitArgs;
///     type Output = InitOutput;
///
///     fn from_args(args: Self::Args) -> Self {
///         Self { args }
///     }
///
///     fn interactive(&mut self) -> Result<()> {
///         // Collect missing arguments interactively
///         if self.args.path.is_none() {
///             let path = Text::new("Path:").prompt()?;
///             self.args.path = Some(path);
///         }
///         Ok(())
///     }
///
///     fn run(self) -> Result<Self::Output> {
///         // Execute the command logic
///         let graph = Graph::init(&self.args.path)?;
///         Ok(InitOutput {
///             name: graph.name().to_string(),
///             path: graph.path().display().to_string(),
///         })
///     }
///
///     fn format_output(&self, output: &Self::Output) {
///         // Format output for humans
///         self.args.global.print(&format!(
///             "Initialized graph {} at {}",
///             output.name, output.path
///         ));
///     }
/// }
/// ```
pub trait Command: Sized {
    /// The argument type for this command
    type Args;

    /// The output type - must be serializable for JSON output
    type Output: serde::Serialize;

    /// Create a command instance from parsed arguments
    fn from_args(args: Self::Args) -> Self;

    /// Get reference to global args for checking flags
    fn global_args(&self) -> &GlobalArgs;

    /// Collect missing arguments interactively
    ///
    /// This is called before `run()` if not in JSON mode.
    /// Override this method to add interactive prompts for missing arguments.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    ///
    /// Returns an error if interactive input fails or is cancelled
    fn interactive(&mut self) -> Result<()> {
        // Default: no interactive mode needed
        Ok(())
    }

    /// Execute the command logic and return structured output
    ///
    /// This is where the main command logic lives. Return a structured
    /// output that can be serialized to JSON.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Output>` - The command's structured output
    ///
    /// # Errors
    ///
    /// Returns an error if command execution fails
    fn run(self) -> Result<Self::Output>;

    /// Format the output for human consumption
    ///
    /// This is called when not in JSON mode. Use `GlobalArgs` helper
    /// methods to print output.
    ///
    /// # Arguments
    ///
    /// * `output` - The structured output from `run()`
    /// * `global` - Reference to global args for printing
    fn format_output(output: &Self::Output, global: &GlobalArgs);

    /// Execute the command (orchestrates interactive, run, and output)
    ///
    /// This is provided by the trait and orchestrates the execution flow:
    /// 1. Validate arguments for JSON mode
    /// 2. Enter interactive mode if needed
    /// 3. Run the command
    /// 4. Output in appropriate format
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    ///
    /// Returns an error if any step fails
    fn execute(mut self) -> Result<()>
    where
        Self: Sized,
    {
        let is_json = self.global_args().json;
        let global = self.global_args().clone();

        // Interactive mode only if not in JSON mode
        if !is_json {
            self.interactive()?;
        }

        // Run the command to get structured output
        let output = self.run()?;

        // Output in appropriate format
        if is_json {
            global.print_json(&output)?;
        } else {
            Self::format_output(&output, &global);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_args_quiet_suppresses_output() {
        let args = GlobalArgs {
            json: false,
            graph: None,
            verbose: false,
            quiet: true,
        };

        // These should not panic, just not print
        args.print("test");
        args.print_verbose("verbose test");
    }

    #[test]
    fn test_global_args_verbose_shows_extra_output() {
        let args = GlobalArgs {
            json: false,
            graph: None,
            verbose: true,
            quiet: false,
        };

        // This would print in real usage, but we can't test output easily
        // Just verify it doesn't panic
        args.print_verbose("verbose test");
    }
}

//! CLI command definitions and the [`Command`] trait.
//!
//! This module defines the trait that all CLI commands implement, providing
//! a consistent pattern for argument handling, execution, and output.

use miette::Result;

use crate::{common::OutputArgs, printer::Printer};

pub mod space;

/// A CLI command that can be executed.
///
/// This trait defines the lifecycle of a command:
///
/// 1. **Construction** - Create the command with parsed arguments via [`new`](Self::new)
/// 2. **Interactive prompting** - Gather missing arguments via [`interactive`](Self::interactive)
/// 3. **Execution** - Perform the command's work via [`execute`](Self::execute)
/// 4. **Output** - Display results via [`finalize`](Self::finalize) or JSON
///
/// The [`run`](Self::run) method orchestrates this lifecycle automatically.
///
/// # Type Parameters
///
/// - `Args` - The clap arguments struct for this command
/// - `Output` - The result type, must be serializable for JSON output
///
/// # Example
///
/// ```ignore
/// struct MyCommand { args: MyArgs }
///
/// impl Command for MyCommand {
///     type Args = MyArgs;
///     type Output = MyOutput;
///
///     fn new(args: Self::Args) -> Self {
///         Self { args }
///     }
///
///     // ... implement other required methods
/// }
/// ```
pub trait Command: Sized {
    /// The argument type for this command, typically a clap `Args` struct.
    type Args;

    /// The output type produced by this command.
    ///
    /// Must implement [`Serialize`](serde::Serialize) for JSON output support.
    type Output: serde::Serialize;

    /// Creates a new command instance from parsed arguments.
    fn new(args: Self::Args) -> Self;

    /// Returns the output formatting arguments.
    fn output_args(&self) -> &OutputArgs;

    /// Creates a printer configured with the command's output settings.
    fn printer(&self) -> Printer {
        self.output_args().printer()
    }

    /// Prompts the user for any missing required arguments.
    ///
    /// This method is called when running in interactive mode (not JSON output).
    /// Implementations should use `inquire` or similar to gather missing values.
    ///
    /// # Errors
    ///
    /// Returns an error if prompting fails or the user cancels.
    async fn interactive(&mut self) -> Result<()>;

    /// Executes the command's main logic.
    ///
    /// This method performs the actual work of the command and returns
    /// the result. It is called after [`interactive`](Self::interactive)
    /// has gathered any missing arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command fails.
    async fn execute(&mut self) -> Result<Self::Output>;

    /// Displays the command's output in human-readable format.
    ///
    /// This method is called after successful execution when not in JSON mode.
    /// Use the [`printer`](Self::printer) to format output consistently.
    fn finalize(&self, output: &Self::Output);

    /// Runs the complete command lifecycle.
    ///
    /// This method orchestrates the command execution:
    ///
    /// 1. If not in JSON mode, calls [`interactive`](Self::interactive) for prompts
    /// 2. Calls [`execute`](Self::execute) to perform the work
    /// 3. Outputs results as JSON or calls [`finalize`](Self::finalize)
    ///
    /// # Errors
    ///
    /// Returns an error if any step fails.
    async fn run(&mut self) -> Result<()> {
        if !self.output_args().json {
            self.interactive().await?;
        }

        let output = self.execute().await?;
        if self.output_args().json {
            self.printer().json(&output)?;
        } else {
            self.finalize(&output);
        }

        Ok(())
    }
}

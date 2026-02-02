//! CLI command definitions and the [`Command`] trait.
//!
//! This module defines the trait that all CLI commands implement, providing
//! a consistent pattern for argument handling, execution, and output.

use miette::Result;

use crate::{common::GlobalArgs, context::Context, printer::Printer};

pub mod space;

/// A CLI command that can be executed.
///
/// This trait defines the lifecycle of a command:
///
/// 1. **Construction** - Create the command with parsed arguments via [`new`](Self::new)
/// 2. **Initialization** - Perform async setup via [`init`](Self::init)
/// 3. **Validation** - Check preconditions via [`validate`](Self::validate)
/// 4. **Interactive prompting** - Gather missing arguments via [`interactive`](Self::interactive)
/// 5. **Execution** - Perform the command's work via [`execute`](Self::execute)
/// 6. **Output** - Display results via [`finalize`](Self::finalize) or JSON
///
/// The [`run`](Self::run) method orchestrates this lifecycle automatically.
///
/// # Type Parameters
///
/// - `'a` - The lifetime of the borrowed context
/// - `Args` - The clap arguments struct for this command
/// - `Output` - The result type, must be serializable for JSON output
///
/// # Example
///
/// ```ignore
/// struct MyCommand<'a> {
///     args: MyArgs,
///     ctx: &'a mut Context,
/// }
///
/// impl<'a> Command<'a> for MyCommand<'a> {
///     type Args = MyArgs;
///     type Output = MyOutput;
///
///     fn new(args: Self::Args, ctx: &'a mut Context) -> Self {
///         Self { args, ctx }
///     }
///
///     fn ctx(&self) -> &Context {
///         self.ctx
///     }
///
///     // ... implement other required methods
/// }
/// ```
pub trait Command<'a>: Sized {
    /// The argument type for this command, typically a clap `Args` struct.
    type Args;

    /// The output type produced by this command.
    ///
    /// Must implement [`Serialize`](serde::Serialize) for JSON output support.
    type Output: serde::Serialize;

    /// Creates a new command instance from parsed arguments and context.
    fn new(args: Self::Args, ctx: &'a mut Context) -> Self;

    /// Returns a reference to the application context.
    fn ctx(&self) -> &Context;

    /// Returns the global arguments.
    fn globals(&self) -> &GlobalArgs;

    /// Creates a printer configured with the command's output settings and theme.
    #[must_use]
    fn printer(&self) -> Printer<'_> {
        Printer::new(
            self.ctx().theme(),
            self.globals().json,
            self.globals().verbose,
            self.globals().quiet,
        )
    }

    /// Performs async initialization before the command runs.
    ///
    /// This method is called at the start of [`run`](Self::run), before
    /// [`interactive`](Self::interactive) or [`execute`](Self::execute).
    /// Use it to load configuration, validate paths, or perform other
    /// async setup that should happen regardless of output mode.
    ///
    /// The default implementation does nothing.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Validates preconditions before interactive prompting or execution.
    ///
    /// This method is called after [`init`](Self::init) but before
    /// [`interactive`](Self::interactive). Use it to verify that the
    /// command can proceed (e.g., required resources exist).
    ///
    /// The default implementation does nothing.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    async fn validate(&mut self) -> Result<()> {
        Ok(())
    }

    /// Indicates whether the command requires interactive prompting.
    fn needs_interaction(&self) -> bool {
        self.globals().interactive
    }

    /// Prompts the user for any missing required arguments.
    ///
    /// This method is called when running in interactive mode (not JSON output).
    /// Implementations should use `inquire` or similar to gather missing values.
    ///
    /// # Errors
    ///
    /// Returns an error if prompting fails or the user cancels.
    async fn interactive(&mut self) -> Result<()> {
        Ok(())
    }

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
    /// 1. Calls [`init`](Self::init) for async initialization
    /// 2. Calls [`validate`](Self::validate) to check preconditions
    /// 3. If not in JSON mode, calls [`interactive`](Self::interactive) for prompts
    /// 4. Calls [`execute`](Self::execute) to perform the work
    /// 5. Outputs results as JSON or calls [`finalize`](Self::finalize)
    ///
    /// # Errors
    ///
    /// Returns an error if any step fails.
    async fn run(&mut self) -> Result<()> {
        self.printer().verbose("Initializing command...");
        self.init().await?;

        self.printer()
            .verbose("Validating command prerequisites...");
        self.validate().await?;

        self.printer().verbose("Checking for interactive mode...");
        if self.needs_interaction() && !self.globals().json {
            self.printer().verbose("Entering interactive mode");
            self.interactive().await?;
        }

        self.printer().verbose("Executing command...");
        let output = self.execute().await?;
        if self.globals().json {
            self.printer().json(&output)?;
        } else {
            self.finalize(&output);
        }

        Ok(())
    }
}

//! CLI command definitions and the [`Command`] trait.
//!
//! This module defines the trait that all CLI commands implement, providing
//! a consistent pattern for argument handling, execution, and output.

use miette::Result;

use crate::{common::GlobalArgs, context::Context, stdin::Stdin};

pub mod space;

/// A CLI command that can be executed.
///
/// This trait defines the lifecycle of a command:
///
/// 1. **Construction** - Create the command with parsed arguments via [`new`](Self::new)
/// 2. **Run** - The [`run`](Self::run) method orchestrates the full lifecycle:
///    a. Creates the application [`Context`]
///    b. Calls [`init`](Self::init) for async initialization
///    c. Calls [`validate`](Self::validate) to check preconditions
///    d. If stdin is piped, calls [`pipe`](Self::pipe) to pre-fill args
///    e. If interactive, calls [`interactive`](Self::interactive) for prompts
///    f. Calls [`execute`](Self::execute) to perform the work
///    g. Outputs results as JSON or calls [`finalize`](Self::finalize)
///
/// # Type Parameters
///
/// - `Args` - The clap arguments struct for this command
/// - `Output` - The result type, must be serializable for JSON output
///
/// # Example
///
/// ```ignore
/// struct MyCommand {
///     args: MyArgs,
/// }
///
/// impl Command for MyCommand {
///     type Args = MyArgs;
///     type Output = MyOutput;
///
///     fn new(args: Self::Args) -> Self {
///         Self { args }
///     }
///
///     fn globals(&self) -> &GlobalArgs {
///         &self.args.globals
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

    /// Returns the global arguments.
    fn globals(&self) -> &GlobalArgs;

    /// Performs async initialization before the command runs.
    ///
    /// This method is called at the start of [`run`](Self::run), after
    /// the context has been created. Use it to load additional data,
    /// validate paths, or perform other async setup.
    ///
    /// The default implementation does nothing.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    async fn init(&mut self, _ctx: &mut Context) -> Result<()> {
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
    async fn validate(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    /// Pre-fills missing arguments from piped stdin lines.
    ///
    /// This method is called after [`validate`](Self::validate) when stdin
    /// is piped (not a TTY). Implementations consume lines positionally
    /// from the iterator to fill in missing `Option` arguments.
    ///
    /// The default implementation does nothing.
    fn pipe(&mut self, _stdin: &mut dyn Iterator<Item = String>) {}

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
    async fn interactive(&mut self, _ctx: &mut Context) -> Result<()> {
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
    async fn execute(&mut self, ctx: &mut Context) -> Result<Self::Output>;

    /// Displays the command's output in human-readable format.
    ///
    /// This method is called after successful execution when not in JSON mode.
    /// Use the context's printer to format output consistently.
    fn finalize(&self, ctx: &Context, output: &Self::Output);

    /// Runs the complete command lifecycle.
    ///
    /// This method orchestrates the command execution:
    ///
    /// 1. Creates the application [`Context`]
    /// 2. Calls [`init`](Self::init) for async initialization
    /// 3. Calls [`validate`](Self::validate) to check preconditions
    /// 4. If stdin is piped, calls [`pipe`](Self::pipe) to pre-fill args
    /// 5. If not in JSON mode, calls [`interactive`](Self::interactive) for prompts
    /// 6. Calls [`execute`](Self::execute) to perform the work
    /// 7. Outputs results as JSON or calls [`finalize`](Self::finalize)
    ///
    /// # Errors
    ///
    /// Returns an error if any step fails.
    async fn run(&mut self) -> Result<()> {
        let mut ctx = Context::load(self.globals()).await?;

        ctx.printer().verbose("Initializing command...");
        self.init(&mut ctx).await?;

        ctx.printer().verbose("Validating command prerequisites...");
        self.validate(&mut ctx).await?;

        let stdin_lines = Stdin::read();
        if stdin_lines.is_piped() {
            ctx.printer()
                .verbose("Reading arguments from piped stdin...");
            self.pipe(&mut stdin_lines.into_iter());
        }

        ctx.printer().verbose("Checking for interactive mode...");
        if self.needs_interaction() && !self.globals().json {
            ctx.printer().info("Entering interactive mode");
            self.interactive(&mut ctx).await?;
        }

        ctx.printer().verbose("Executing command...");
        let output = self.execute(&mut ctx).await?;
        if self.globals().json {
            ctx.printer().json(&output)?;
        } else {
            self.finalize(&ctx, &output);
        }

        Ok(())
    }
}

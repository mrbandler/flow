# Flow CLI

This crate provides the command-line interface for Flow, a note-taking system for developers.

> **📖 Documentation Hub**: This README serves as the primary documentation for the Flow CLI architecture, command structure, and development guidelines. All CLI-related documentation lives here.

## Table of Contents

- [Architecture](#architecture)
- [Global Flags](#global-flags)
- [GlobalArgs Helper Methods](#globalargs-helper-methods)
- [Command Trait](#command-trait)
- [Interactive Mode](#interactive-mode)
- [Adding a New Command](#adding-a-new-command)
- [Command Structure](#command-structure)
- [Testing Commands](#testing-commands)
- [Common Patterns](#common-patterns)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)
- [Migration Guide](#migration-guide)
- [Future Enhancements](#future-enhancements)
- [Examples](#examples)

## Architecture

The CLI is organized into modules for better maintainability:

```
src/
├── lib.rs              # Main CLI entry point and command dispatcher
├── common.rs           # Shared types (GlobalArgs, Command trait)
└── commands/           # Individual command modules
    ├── mod.rs          # Command module exports
    ├── init.rs         # flow init command
    ├── open.rs         # flow open command
    └── add.rs          # flow add command
```

## Global Flags

All commands support these global flags (defined in `common::GlobalArgs`):

- `--json` - Output in JSON format
- `--graph <name|path>` - Target specific graph by name or path (overrides active graph)
- `--verbose`, `-v` - Detailed logging
- `--quiet`, `-q` - Suppress non-error output

These flags are automatically included in every command via the `#[command(flatten)]` attribute.

### GlobalArgs Definition

```rust
#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub graph: Option<String>,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short, long, global = true)]
    pub quiet: bool,
}
```

### Usage Pattern

Every command Args struct must include GlobalArgs using `#[command(flatten)]`:

```rust
#[derive(Args)]
pub struct MyCommandArgs {
    #[command(flatten)]
    pub global: GlobalArgs,
    
    // Command-specific arguments
    pub my_arg: String,
}
```

The `#[command(flatten)]` attribute tells clap to merge GlobalArgs fields directly into the command's arguments, making them available as top-level flags.

## GlobalArgs Helper Methods

### `load_graph() -> Result<Graph>`

Loads the target graph respecting the `--graph` flag:

```rust
// If --graph is specified (as name or path), use that
// Otherwise, load the active graph from config
let graph = args.global.load_graph()?;
```

**How it works:**
1. If `--graph <value>` is provided:
   - First checks if `<value>` is a registered graph name
   - If not found by name, checks if it's a registered graph path
   - If still not found, treats it as a direct file path
2. If no `--graph` flag, uses the active graph from config

**Benefits:**
- Single method to get the correct graph
- Accepts both graph names and paths
- Consistent error messages
- Respects user's explicit `--graph` override

**When to use:** Any command that needs to work with a graph should use this method.

### `print(&self, message: &str)`

Prints output respecting the `--quiet` and `--json` flags:

```rust
args.global.print("Operation completed successfully");
```

**When to use:**
- Normal command output
- Success messages
- Results that users expect to see

**Note:** This method outputs nothing when `--json` is set, as JSON output should be handled via `print_json()`.

### `print_verbose(&self, message: &str)`

Prints verbose debugging information (only with `--verbose`, suppressed with `--json`):

```rust
args.global.print_verbose("Loading configuration from disk");
args.global.print_verbose("Found 3 registered graphs");
```

**When to use:**
- Debug information
- Step-by-step operation details
- Information helpful for troubleshooting

**Note:** This method outputs nothing when `--json` is set.

### `print_error(&self, message: &str)`

Prints error messages (always shown unless `--quiet` or `--json`):

```rust
args.global.print_error("Failed to connect to server");
```

**When to use:**
- Error conditions
- Warnings
- Critical information that shouldn't be silenced by default

**Note:** When `--json` is set, errors should be handled through the `Result` error chain instead of printing directly.

### `print_json<T: Serialize>(&self, value: &T) -> Result<()>`

Prints JSON output (only when `--json` flag is set):

```rust
use serde::Serialize;

#[derive(Serialize)]
struct MyOutput {
    name: String,
    count: usize,
}

let output = MyOutput {
    name: "example".to_string(),
    count: 42,
};

args.global.print_json(&output)?;
```

**When to use:**
- Structured output that should be JSON when `--json` flag is set
- Command results that need to be machine-readable

**How it works:**
- Only outputs when `--json` flag is set
- Automatically pretty-prints the JSON
- Returns an error if serialization fails

**Pattern:** Call both `print_json()` and `print()` in your command. The appropriate one will output based on flags:

```rust
// Both methods are called, but only one outputs
args.global.print_json(&output)?;
args.global.print("Operation completed successfully");
```

## Helper Utilities

### `path_to_display_string(path: &Path) -> String`

Converts a canonicalized path to a clean display string. Available from `crate::common`.

```rust
use crate::common::path_to_display_string;

let canonical_path = some_path.canonicalize()?;
let display_path = path_to_display_string(&canonical_path);
args.global.print(&format!("Path: {}", display_path));
```

**Why needed:**
On Windows, canonicalized paths include the `\\?\` prefix (e.g., `\\?\D:\path\to\file`), which looks odd in output. This function strips that prefix for cleaner display across all platforms.

**When to use:**
- When displaying canonicalized paths to users
- In JSON output with path fields
- Any user-facing path output

## Interactive Mode

Flow CLI commands support interactive mode when optional arguments are not provided. This follows the design principle of "Dual Mode Operation" from the CLI spec.

Interactive mode is powered by the [`inquire`](https://crates.io/crates/inquire) crate, providing a modern, user-friendly terminal UI with:
- Beautiful, cross-platform interface
- Fuzzy search for selections
- Clear prompts with help messages
- Keyboard navigation (arrow keys, vim-style hjkl)
- Default value suggestions

### When Interactive Mode Activates

Commands enter interactive mode when:
- Required arguments are optional (wrapped in `Option<T>`)
- The argument is not provided by the user
- The command can present meaningful choices to the user

### Current Interactive Commands

#### `flow open` (no arguments)

When you run `flow open` without specifying a graph, an interactive selection menu appears:

```bash
$ flow open
? Select a graph to open: ›
❯ personal (/home/user/notes) [active]
  work (/home/user/work/flow)
  archive (/mnt/archive/old-notes)
```

Features:
- Shows `[active]` indicator for currently active graph
- Navigate with arrow keys or vim-style hjkl
- Fuzzy search by typing
- Press Enter to confirm selection
- ESC or Ctrl+C to cancel

#### `flow init` (no arguments)

When you run `flow init` without specifying a path, it prompts for input:

```bash
$ flow init
? Directory path: › .
  Path where the graph will be initialized
? Graph name: › 
  Leave empty to use directory name
```

Features:
- Shows default values inline
- Provides help messages below each prompt
- Press Enter to accept default
- ESC or Ctrl+C to cancel

### Interactive Mode Design

Interactive prompts use the `inquire` library and provide:
- Modern, colorful terminal UI
- Clear instructions and help messages
- Inline default value display
- Fuzzy search for selections (type to filter)
- Keyboard navigation (arrows, vim keys)
- Cancellation support (ESC or Ctrl+C)
- Input validation before proceeding
- Helpful error messages for invalid input

### Adding Interactive Mode to Commands

To add interactive mode to a command:

1. **Make arguments optional**:
   ```rust
   pub struct MyArgs {
       #[command(flatten)]
       pub global: GlobalArgs,
       
       /// Description (enters interactive mode if not provided)
       pub my_arg: Option<String>,
   }
   ```

2. **Handle None case with prompts using `inquire`**:
   ```rust
   use inquire::{Select, Text};
   
   fn execute(self) -> Result<()> {
       let my_value = if let Some(value) = self.args.my_arg {
           value
       } else {
           // Enter interactive mode
           args.global.print_verbose("Entering interactive mode");
           
           // For text input:
           Text::new("Prompt:")
               .with_default("default value")
               .with_help_message("Optional help text")
               .prompt()
               .map_err(|_| anyhow::anyhow!("Input cancelled"))?
           
           // For selections:
           // let options = vec!["Option 1", "Option 2"];
           // Select::new("Choose:", options)
           //     .prompt()
           //     .map_err(|_| anyhow::anyhow!("Selection cancelled"))?
       };
       
       // Continue with value...
       Ok(())
   }
   ```

3. **Suppress interactive output with --json**:
   Interactive prompts should check if JSON mode is active and fail gracefully:
   ```rust
   if args.global.json && self.args.my_arg.is_none() {
       anyhow::bail!("Argument required in JSON mode");
   }
   ```

## Command Trait

The `Command` trait provides a clean separation of concerns for CLI commands:

```rust
pub trait Command: Sized {
    type Args;
    type Output: Serialize;
    
    fn from_args(args: Self::Args) -> Self;
    fn global_args(&self) -> &GlobalArgs;
    fn interactive(&mut self) -> Result<()>;
    fn run(self) -> Result<Self::Output>;
    fn format_output(output: &Self::Output, global: &GlobalArgs);
    fn execute(mut self) -> Result<()>; // Provided by trait
}
```

### Benefits

- **Separation of Concerns** - Interactive, logic, and output are separate methods
- **Type Safety** - Structured output types ensure consistency
- **Automatic JSON** - JSON output handled by the trait automatically
- **Clean Code** - Each method has a single, clear responsibility
- **Testability** - Easy to test `run()` logic independently
- **Consistent UX** - All commands follow the same execution flow

### Execution Flow

When a command is executed, the trait orchestrates:

1. **`from_args`** - Create command from parsed CLI arguments
2. **`interactive`** - Collect missing arguments (only if not in `--json` mode)
3. **`run`** - Execute command logic, return structured output
4. **Output** - Format as JSON (with `--json`) or human-readable (via `format_output`)

This separation means your command logic (`run`) is clean and focused:
- No if/else for JSON vs text output
- No interactive prompts mixed with business logic
- Returns structured data that works for both output formats

### Required Methods

#### `from_args(args: Self::Args) -> Self`
Create command instance from parsed arguments.

#### `global_args(&self) -> &GlobalArgs`
Return reference to global args for checking flags.

#### `run(self) -> Result<Self::Output>`
Execute the command logic and return structured output. This is where your main business logic lives.

**Guidelines:**
- Validate required arguments first (they may be None if user skipped interactive mode)
- Use `self.args.global.print_verbose()` for debug logging
- Return structured `Output` type (must be serializable)
- Don't format output here - just return data
- Don't check `--json` flag - trait handles it

**Example validation:**
```rust
fn run(self) -> Result<Self::Output> {
    // Validate required arguments
    let path = self.args.path
        .ok_or_else(|| anyhow::anyhow!("Path argument is required"))?;
    
    // Continue with business logic...
}
```

#### `format_output(output: &Self::Output, global: &GlobalArgs)`
Format output for human consumption. Called when not in JSON mode.

**Guidelines:**
- Use `global.print()` methods for output
- Format the structured output for readability
- Don't access command args (static method)
- All needed data should be in `output`

### Optional Methods

#### `interactive(&mut self) -> Result<()>`
Collect missing arguments interactively. Override this to add interactive prompts.

**Guidelines:**
- Only called when not in `--json` mode
- Modify `self.args` to fill in missing values
- Use `inquire` for prompts (Text, Select, etc.)
- Return error if cancelled

### Command Dispatch Pattern

Commands are dispatched directly using the Command trait in `lib.rs`:

```rust
pub fn run(cmd: Commands) -> Result<()> {
    use crate::common::Command;
    
    match cmd {
        Commands::MyCommand(args) => commands::mycommand::MyCommand::from_args(args).execute(),
    }
}
```

This pattern:
- Uses the Command trait directly - no wrapper functions needed
- Keeps code simple and explicit
- Maintains type safety through the trait

## Adding a New Command

Each command is self-contained in its own module. Follow these steps to add a new command:

### 1. Create a new file in `src/commands/`

For example, to add a `flow list` command, create `src/commands/list.rs`:

```rust
//! List all registered graphs.

use anyhow::Result;
use clap::Args;
use flow_core::config::Config;
use crate::common::{Command, GlobalArgs};

/// Arguments for the list command.
#[derive(Args)]
pub struct ListArgs {
    /// Global flags (--json, --verbose, etc.)
    #[command(flatten)]
    pub global: GlobalArgs,
    
    /// Show detailed information
    #[arg(short = 'd', long)]
    pub detailed: bool,
}

/// List command implementation.
pub struct ListCommand {
    args: ListArgs,
}

impl Command for ListCommand {
    type Args = ListArgs;
    
    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }
    
    fn execute(self) -> Result<()> {
        let args = self.args;
        
        args.global.print_verbose("Loading configuration");
        let config = Config::load()?;
        
        // Implementation here...
        args.global.print("Listed all graphs");
        
        Ok(())
    }
}

```

### 2. Export the module in `src/commands/mod.rs`

Add your module to the list:

```rust
pub mod add;
pub mod init;
pub mod list;  // <- Add this
pub mod open;
```

### 3. Add the command variant to `src/lib.rs`

In the `Commands` enum:

```rust
#[derive(Subcommand)]
pub enum Commands {
    Init(commands::init::InitArgs),
    Open(commands::open::OpenArgs),
    Add(commands::add::AddArgs),
    List(commands::list::ListArgs),  // <- Add this
}
```

### 4. Add the dispatch case in `src/lib.rs`

In the `run()` function's match statement:

```rust
pub fn run(cmd: Commands) -> Result<()> {
    use crate::common::Command;
    
    match cmd {
        Commands::Init(args) => commands::init::InitCommand::from_args(args).execute(),
        Commands::Open(args) => commands::open::OpenCommand::from_args(args).execute(),
        Commands::Add(args) => commands::add::AddCommand::from_args(args).execute(),
        Commands::List(args) => commands::list::ListCommand::from_args(args).execute(),  // <- Add this
    }
}
```

### 5. Build and test

```bash
cargo build
./target/debug/flow list --help
./target/debug/flow list
```

## Command Structure

Each command module should follow this pattern:

```rust
//! Brief description of what the command does.

use anyhow::Result;
use clap::Args;
use serde::Serialize;
use crate::common::{Command, GlobalArgs};

/// Output structure for the command.
#[derive(Debug, Clone, Serialize)]
pub struct MyCommandOutput {
    pub result: String,
    pub count: usize,
}

/// Arguments for the command.
#[derive(Args)]
pub struct MyCommandArgs {
    /// Global flags (always include this!)
    #[command(flatten)]
    pub global: GlobalArgs,
    
    /// Optional argument (enables interactive mode)
    pub my_arg: Option<String>,
}

/// Command implementation.
pub struct MyCommand {
    args: MyCommandArgs,
}

impl Command for MyCommand {
    type Args = MyCommandArgs;
    type Output = MyCommandOutput;
    
    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }
    
    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }
    
    // Optional: Implement if command supports interactive mode
    fn interactive(&mut self) -> Result<()> {
        if self.args.my_arg.is_none() {
            let value = Text::new("Enter value:")
                .with_default("default")
                .prompt()?;
            self.args.my_arg = Some(value);
        }
        Ok(())
    }
    
    // Required: Execute command logic and return structured output
    fn run(self) -> Result<Self::Output> {
        self.args.global.print_verbose("Starting operation");
        
        // Validate required arguments (may be None if interactive was skipped)
        let my_arg = self.args.my_arg
            .ok_or_else(|| anyhow::anyhow!("Argument is required"))?;
        
        // Do the actual work
        let result = format!("Processed: {}", my_arg);
        
        Ok(MyCommandOutput {
            result,
            count: 42,
        })
    }
    
    // Required: Format output for human consumption
    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.print(&format!("Result: {}", output.result));
        global.print(&format!("Count: {}", output.count));
    }
}
```

### Complete Example

Here's a complete example showing all the patterns:

```rust
//! Show information about a node.

use anyhow::Result;
use clap::Args;
use serde::Serialize;
use flow_core::graph::Graph;
use crate::common::{Command, GlobalArgs};

/// Output for the show command.
#[derive(Debug, Clone, Serialize)]
pub struct ShowOutput {
    pub node_id: String,
    pub content: String,
    pub created: Option<String>,
    pub modified: Option<String>,
}

/// Arguments for the show command.
#[derive(Args)]
pub struct ShowArgs {
    #[command(flatten)]
    pub global: GlobalArgs,
    
    /// Node ID to show
    pub node_id: String,
    
    /// Include metadata
    #[arg(short, long)]
    pub metadata: bool,
}

/// Show command implementation.
pub struct ShowCommand {
    args: ShowArgs,
}

impl Command for ShowCommand {
    type Args = ShowArgs;
    type Output = ShowOutput;
    
    fn from_args(args: Self::Args) -> Self {
        Self { args }
    }
    
    fn global_args(&self) -> &GlobalArgs {
        &self.args.global
    }
    
    fn run(self) -> Result<Self::Output> {
        self.args.global.print_verbose(&format!("Looking up node: {}", self.args.node_id));
        
        // Load the graph (respects --graph flag)
        let graph = self.args.global.load_graph()?;
        self.args.global.print_verbose("Graph loaded successfully");
        
        // Get the node
        let node = graph.get_node(&self.args.node_id)?;
        
        Ok(ShowOutput {
            node_id: self.args.node_id.clone(),
            content: node.content.clone(),
            created: if self.args.metadata { Some(node.created.clone()) } else { None },
            modified: if self.args.metadata { Some(node.modified.clone()) } else { None },
        })
    }
    
    fn format_output(output: &Self::Output, global: &GlobalArgs) {
        global.print(&format!("Content: {}", output.content));
        
        if let Some(created) = &output.created {
            global.print(&format!("Created: {}", created));
        }
        if let Some(modified) = &output.modified {
            global.print(&format!("Modified: {}", modified));
        }
    }
}
```

## Testing Commands

### Testing Pattern

The Command trait makes testing easier:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_show_command() {
        let args = ShowArgs {
            global: GlobalArgs {
                json: false,
                graph: Some("test-graph".to_string()),
                verbose: false,
                quiet: true, // Suppress output in tests
            },
            node_id: "test-node".to_string(),
            metadata: false,
        };
        
        let result = ShowCommand::from_args(args).execute();
        assert!(result.is_ok());
    }
}
```

### Manual Testing

Test your command:

```bash
# Build
cargo build

# Run command
./target/debug/flow your-command --help
./target/debug/flow your-command [args]

# Test with global flags
./target/debug/flow your-command --verbose
./target/debug/flow your-command --quiet
./target/debug/flow your-command --graph "My Graph"

# Run tests (when you add them)
cargo test
```

## Common Patterns

### Loading Active Graph

```rust
// Simple - use the active graph or --graph override
let graph = args.global.load_graph()?;
```

### Conditional Output

```rust
// Always respect --quiet flag
args.global.print("User-facing output");

// Debug information only with --verbose
args.global.print_verbose("Internal details");

// Errors should always be visible (unless --quiet)
args.global.print_error("Something went wrong");
```

### Using Graph Name or Path

```rust
// The global --graph flag accepts both names and paths
// Examples:
// flow mycommand --graph "My Graph"        (by name)
// flow mycommand --graph ./path/to/graph  (by path)

// In your command, just use load_graph()
let graph = args.global.load_graph()?;

// It automatically handles:
// 1. Name lookup in registered graphs
// 2. Path lookup in registered graphs
// 3. Direct path resolution if not registered
// 4. Falling back to active graph if no --graph flag
```

### JSON Output Pattern

```rust
use serde::Serialize;

#[derive(Serialize)]
struct CommandOutput {
    field1: String,
    field2: usize,
}

fn execute(self) -> Result<()> {
    // ... do work ...
    
    let output = CommandOutput {
        field1: "value".to_string(),
        field2: 42,
    };
    
    // Call both - the appropriate one outputs based on flags
    args.global.print_json(&output)?;
    args.global.print("Human-readable output");
    
    Ok(())
}
```

### Progress Reporting

```rust
args.global.print_verbose("Step 1: Loading configuration");
// ... do work ...

args.global.print_verbose("Step 2: Processing nodes");
// ... do work ...

args.global.print_verbose("Step 3: Saving changes");
// ... do work ...

args.global.print("Operation completed successfully");
```

### Clap Argument Attributes

Common attributes for command arguments:

- `#[command(flatten)]` - Include GlobalArgs in your command
- `#[arg(short, long)]` - Adds both short (`-v`) and long (`--verbose`) flags
- `#[arg(short = 'v', long)]` - Custom short flag
- `#[arg(long)]` - Long flag only
- `#[arg(default_value = "value")]` - Default value if not provided
- `#[arg(value_name = "NAME")]` - Display name in help text
- `#[arg(help = "Description")]` - Help text (or use doc comments `///`)
- `#[arg(global = true)]` - Available for all subcommands (used in GlobalArgs)

## Error Handling

Use `anyhow::Result` for error handling. Prefer meaningful error messages:

```rust
// Good
anyhow::bail!("Graph '{}' not found in configuration", name);

// Also good
Err(anyhow::anyhow!("Failed to load graph: {}", e))?;

// Less helpful
anyhow::bail!("Error");
```

Commands return `Result<()>` - errors bubble up to the main binary for handling.

## Best Practices

### Do's ✅

1. **Always include GlobalArgs** - Use `#[command(flatten)]` in every command's Args struct
2. **Implement all Command trait methods** - `global_args`, `run`, and `format_output` are required
3. **Return structured Output** - Make Output types serializable with `#[derive(Serialize)]`
4. **Separate concerns** - Keep interactive, logic, and formatting in separate methods
5. **Use `global.print()` methods** - Never use println!/eprintln! directly
6. **Provide verbose logging** - Use `print_verbose()` in `run()` for debugging
7. **Use `global.load_graph()`** - Handles both names and paths automatically
8. **Document your types** - Use doc comments (`///`) for Args, Output, and all fields
9. **Test with all flag combinations** - Test --quiet, --verbose, --graph, --json, etc.
10. **Keep commands focused** - Each command should do one thing well
11. **Handle errors gracefully** - Provide helpful, context-rich error messages
12. **Follow the spec** - Refer to `.spec/cli.md` for command specifications

### Don'ts ❌

1. **Don't use `println!()` or `eprintln!()` directly** - Use global.print() methods
2. **Don't put output logic in `run()`** - Return structured data, format in `format_output()`
3. **Don't ignore the --quiet or --json flags** - The trait handles this automatically
4. **Don't hardcode graph loading** - Use global.load_graph() (handles names and paths)
5. **Don't skip verbose logging** - It's invaluable for debugging
6. **Don't make GlobalArgs optional** - Every command needs it
7. **Don't bypass the trait** - Always implement Command properly
8. **Don't write generic error messages** - Be specific about what went wrong
9. **Don't access self.args in `format_output()`** - It's a static method, use output fields instead

## Future Enhancements

The Command trait enables future features like:

### Command Middleware

```rust
pub trait CommandMiddleware {
    fn before_execute(&self, cmd: &dyn Command) -> Result<()>;
    fn after_execute(&self, cmd: &dyn Command, result: &Result<()>) -> Result<()>;
}
```

### Command Composition

```rust
pub trait ComposableCommand: Command {
    fn chain<T: Command>(self, next: T) -> CommandChain<Self, T>;
}
```

### Automatic JSON Output

```rust
impl Command for MyCommand {
    fn execute(self) -> Result<()> {
        let result = // ... do work ...
        
        if self.args.global.json {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            self.args.global.print(&format!("{}", result));
        }
        
        Ok(())
    }
}
```

## Examples

See existing commands for examples:
- `init.rs` - Simple command with validation and GlobalArgs, JSON output, interactive mode
- `open.rs` - Complex logic with multiple code paths, verbose logging, interactive graph selection
- `add.rs` - Uses `global.load_graph()` to get active graph
- `clean.rs` - Iterates over configuration, supports --dry-run flag, JSON output
- `common.rs` - Defines GlobalArgs and Command trait

## Summary

The Command trait and GlobalArgs pattern provides:

- ✅ **Consistency** - All commands follow the same structure
- ✅ **Type Safety** - Compile-time guarantees
- ✅ **Testability** - Easy to test in isolation
- ✅ **Global Flags** - Shared functionality across all commands
- ✅ **Future Proof** - Easy to extend with new features
- ✅ **User Experience** - Consistent behavior with --quiet, --verbose, etc.
- ✅ **Maintainability** - Clear patterns for new contributors

By following this pattern, you ensure that Flow CLI commands are consistent, maintainable, and user-friendly.

//! Error types for Flow CLI with beautiful diagnostics.

use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

/// CLI-specific errors with diagnostic information.
#[derive(Error, Debug, Diagnostic)]
pub enum CliError {
    /// Graph not found error
    #[error("Graph not found")]
    #[diagnostic(
        code(flow::graph::not_found),
        help("Try one of these:\n  • List available graphs: flow list\n  • Initialize a new graph: flow init <path>\n  • Use an absolute path if specifying by path")
    )]
    GraphNotFound {
        /// The graph name or path that was not found
        graph: String,
    },

    /// Graph already exists error
    #[error("Graph already exists")]
    #[diagnostic(
        code(flow::graph::already_exists),
        help("Choose a different path or use 'flow open' to open the existing graph")
    )]
    GraphAlreadyExists {
        /// The path where the graph already exists
        path: PathBuf,
    },

    /// No active graph set
    #[error("No active graph")]
    #[diagnostic(
        code(flow::graph::no_active),
        help("Set an active graph using one of these:\n  • Open a graph: flow open <name|path>\n  • Initialize a new graph: flow init <path>")
    )]
    NoActiveGraph,

    /// Required argument missing
    #[error("Missing required argument: {argument}")]
    #[diagnostic(
        code(flow::arg::required),
        help("Provide the argument or run without --json to use interactive mode")
    )]
    MissingArgument {
        /// The name of the missing argument
        argument: String,
    },

    /// Path does not exist
    #[error("Path does not exist")]
    #[diagnostic(code(flow::path::not_found), help("Check the path and try again"))]
    PathNotFound {
        /// The path that was not found
        path: PathBuf,
    },

    /// Invalid graph structure
    #[error("Invalid graph structure")]
    #[diagnostic(
        code(flow::graph::invalid),
        help("The directory exists but doesn't contain a valid Flow graph.\nUse 'flow init' to initialize a graph in this directory.")
    )]
    InvalidGraph {
        /// The path that doesn't contain a valid graph
        path: PathBuf,
    },

    /// Configuration error
    #[error("Configuration error")]
    #[diagnostic(code(flow::config::error))]
    ConfigError {
        /// The underlying error message
        message: String,
    },

    /// IO error
    #[error("IO error")]
    #[diagnostic(code(flow::io::error))]
    IoError {
        /// The path where the IO error occurred
        path: Option<PathBuf>,
        /// The underlying error
        #[source]
        source: std::io::Error,
    },

    /// Interactive mode cancelled
    #[error("Operation cancelled")]
    #[diagnostic(code(flow::interactive::cancelled))]
    InteractiveCancelled,

    /// Generic error
    #[error("{message}")]
    #[diagnostic(code(flow::error))]
    Other {
        /// The error message
        message: String,
    },
}

impl CliError {
    /// Create a GraphNotFound error
    pub fn graph_not_found(graph: impl Into<String>) -> Self {
        Self::GraphNotFound {
            graph: graph.into(),
        }
    }

    /// Create a GraphAlreadyExists error
    pub fn graph_already_exists(path: impl Into<PathBuf>) -> Self {
        Self::GraphAlreadyExists { path: path.into() }
    }

    /// Create a PathNotFound error
    pub fn path_not_found(path: impl Into<PathBuf>) -> Self {
        Self::PathNotFound { path: path.into() }
    }

    /// Create an InvalidGraph error
    pub fn invalid_graph(path: impl Into<PathBuf>) -> Self {
        Self::InvalidGraph { path: path.into() }
    }

    /// Create a MissingArgument error
    pub fn missing_argument(argument: impl Into<String>) -> Self {
        Self::MissingArgument {
            argument: argument.into(),
        }
    }

    /// Create an IoError
    pub fn io_error(source: std::io::Error, path: Option<PathBuf>) -> Self {
        Self::IoError { path, source }
    }

    /// Create a ConfigError
    pub fn config_error(message: String) -> Self {
        Self::ConfigError { message }
    }
}

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, CliError>;

// Implement From for common error types
impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            path: None,
            source: err,
        }
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        Self::Other {
            message: err.to_string(),
        }
    }
}

impl From<inquire::InquireError> for CliError {
    fn from(_err: inquire::InquireError) -> Self {
        Self::InteractiveCancelled
    }
}

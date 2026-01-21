//! # flow-core
//!
//! Core abstractions for the Flow notes and outliner system.
//!
//! This crate provides the fundamental building blocks for managing
//! notes, spaces, and file system operations. It is designed to be
//! the foundation that other Flow crates build upon.
//!
//! ## Key Concepts
//!
//! - **[`Space`]** - A workspace containing notes, configuration, and metadata.
//!   Spaces are the top-level organizational unit in Flow.
//!
//! - **[`Locator`]** - A flexible way to identify spaces, either by their
//!   human-readable name or by an explicit filesystem path.
//!
//! - **`Filesystem`** - An abstraction over file I/O operations, enabling
//!   testability and potential future support for different storage backends.
//!
//! - **[`Error`]** - Rich error types with diagnostic information for
//!   user-friendly error reporting.
//!
//! ## Examples
//!
//! ### Creating a new space
//!
//! ```no_run
//! use std::path::Path;
//! use flow_core::Space;
//!
//! # async fn example() -> miette::Result<()> {
//! let space = Space::init(Path::new("./my-notes"), "personal").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Loading an existing space
//!
//! ```no_run
//! use flow_core::Space;
//!
//! # async fn example() -> miette::Result<()> {
//! // Load by name (resolved from configuration)
//! let space = Space::load("personal").await?;
//!
//! // Or load by explicit path
//! let space = Space::load(std::path::PathBuf::from("./my-notes")).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Error handling
//!
//! ```no_run
//! use std::path::Path;
//! use flow_core::{Space, Error};
//!
//! # async fn example() -> miette::Result<()> {
//! match Space::init(Path::new("./my-notes"), "personal").await {
//!     Ok(space) => println!("Space created successfully!"),
//!     Err(report) => {
//!         // Errors are miette::Report, providing rich diagnostics
//!         eprintln!("Failed to create space: {:?}", report);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Structure
//!
//! - `space` - Space management, including initialization and loading.
//! - `filesystem` - Filesystem abstraction layer for I/O operations.
//! - `errors` - Error types with rich diagnostic information.
//!
//! ## Feature Flags
//!
//! This crate does not currently define any feature flags.
//!
//! ## Re-exports
//!
//! For convenience, the most commonly used types are re-exported at the
//! crate root:
//!
//! - [`Space`] - The main space type.
//! - [`Locator`] - For identifying spaces by name or path.
//! - [`Error`] - The error type for this crate.
//!
//! [`Space`]: space::Space
//! [`Locator`]: space::Locator

mod filesystem;

mod config;
mod errors;
mod space;

pub use self::config::Config;
pub use self::errors::Error;
pub use self::space::Locator;
pub use self::space::Space;

//! Filesystem abstraction layer for Flow.
//!
//! This module provides a trait-based abstraction over filesystem operations,
//! allowing for different implementations such as local filesystem access,
//! in-memory filesystems for testing, or remote storage backends.
//!
//! # Design
//!
//! The [`Filesystem`] trait defines the core operations needed by Flow:
//! reading and writing files. By abstracting these operations behind a trait,
//! we gain several benefits:
//!
//! - **Testability**: Tests can use an in-memory implementation to avoid
//!   touching the real filesystem.
//! - **Flexibility**: Future implementations could support cloud storage,
//!   encrypted filesystems, or other backends.
//! - **Isolation**: The core logic doesn't depend on `std::fs` directly.
//!
//! # Implementations
//!
//! This module provides the following implementations of the [`Filesystem`] trait:
//!
//! - [`LocalFilesystem`] - The production implementation that delegates to
//!   Tokio's async filesystem APIs for local file I/O.
//!
//! # Re-exports
//!
//! The following types are re-exported for convenience:
//!
//! - [`Filesystem`] - The core trait defining filesystem operations.
//! - [`LocalFilesystem`] - The local filesystem implementation.
//!
//! # Examples
//!
//! ```ignore
//! use std::path::Path;
//! use flow_core::filesystem::{Filesystem, LocalFilesystem};
//!
//! # async fn example() -> miette::Result<()> {
//! let fs = LocalFilesystem;
//!
//! // Write a file
//! fs.write(Path::new("notes/todo.md"), "# TODO\n- Buy milk").await?;
//!
//! // Read it back
//! let content = fs.read(Path::new("notes/todo.md")).await?;
//! let text = String::from_utf8(content).expect("valid UTF-8");
//! assert!(text.contains("Buy milk"));
//! # Ok(())
//! # }
//! ```

mod local;
mod traits;

pub use self::local::LocalFilesystem;
pub use self::traits::Filesystem;

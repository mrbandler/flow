//! I/O error type for filesystem operations.

use miette::Diagnostic;
use thiserror::Error;

/// A filesystem operation failed.
///
/// This type wraps low-level I/O errors from the operating system,
/// such as permission denied, disk full, or network errors when
/// accessing remote filesystems.
///
/// # Error Code
///
/// `flow::io_error`
///
/// # Examples
///
/// ```
/// use flow_errors::IoError;
/// use std::io;
///
/// let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
/// let error: IoError = io_error.into();
/// ```
#[derive(Debug, Error, Diagnostic)]
#[error("Filesystem error: {0}")]
#[diagnostic(code(flow::io_error))]
pub struct IoError(#[from] pub std::io::Error);

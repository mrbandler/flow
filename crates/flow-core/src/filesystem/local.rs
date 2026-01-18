//! Local filesystem implementation.
//!
//! This module provides [`LocalFilesystem`], the production implementation
//! of the [`Filesystem`](super::Filesystem) trait that delegates to
//! Tokio's async filesystem APIs.

use std::path::Path;

use miette::Result;
use tokio::fs::{create_dir, metadata, read, read_dir, read_to_string, try_exists, write};

use crate::errors::Error;
use crate::filesystem::traits::Filesystem;

/// A [`Filesystem`] implementation that operates on the local filesystem.
///
/// This is the standard implementation used in production. It delegates
/// to the operating system's filesystem APIs via Tokio's async filesystem
/// operations.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// use flow_core::filesystem::{Filesystem, LocalFilesystem};
///
/// # async fn example() -> miette::Result<()> {
/// let fs = LocalFilesystem;
///
/// // Check if a file exists
/// if fs.exists(Path::new("notes/todo.md")).await? {
///     let content = fs.read(Path::new("notes/todo.md")).await?;
///     println!("File has {} bytes", content.len());
/// }
///
/// // Write a new file
/// fs.write(Path::new("notes/new.md"), b"# New Note").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalFilesystem;

impl Filesystem for LocalFilesystem {
    /// Checks if a path exists using [`tokio::fs::try_exists`].
    ///
    /// This is a non-blocking operation that queries the filesystem
    /// asynchronously.
    async fn exists(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool> {
        try_exists(&path).await.map_err(|e| Error::Io(e).into())
    }

    /// Checks if a path is a directory using [`tokio::fs::metadata`].
    ///
    /// Returns an error if the path does not exist, unlike [`exists`](Self::exists)
    /// which returns `false` for non-existent paths.
    async fn is_dir(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool> {
        let metadata = metadata(&path).await.map_err(Error::Io)?;

        Ok(metadata.is_dir())
    }

    /// Checks if a directory is empty by attempting to read its first entry.
    ///
    /// Uses [`tokio::fs::read_dir`] internally. Returns `false` if the path
    /// is not a directory (does not error).
    async fn is_dir_empty(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool> {
        if !self.is_dir(&path).await? {
            return Ok(false);
        }

        let mut entries = read_dir(&path).await.map_err(Error::Io)?;
        let is_empty = entries.next_entry().await.map_err(Error::Io)?.is_none();

        Ok(is_empty)
    }

    /// Creates a directory using [`tokio::fs::create_dir`].
    ///
    /// This does **not** create parent directories. Use this only when
    /// the parent directory is known to exist.
    async fn create_dir(&self, path: impl AsRef<Path> + Send + Sync) -> Result<()> {
        create_dir(&path).await.map_err(|e| Error::Io(e).into())
    }

    /// Writes content to a file using [`tokio::fs::write`].
    ///
    /// This operation is atomic on most platforms — the file is either
    /// fully written or not modified at all.
    async fn write(
        &self,
        path: impl AsRef<Path> + Send + Sync,
        contents: impl AsRef<[u8]> + Send + Sync,
    ) -> Result<()> {
        write(&path, &contents)
            .await
            .map_err(|e| Error::Io(e).into())
    }

    /// Reads the entire file contents using [`tokio::fs::read`].
    ///
    /// The entire file is read into memory. For large files, consider
    /// using streaming APIs instead.
    async fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path = path.as_ref();
        read(path).await.map_err(|e| Error::Io(e).into())
    }

    /// Reads the entire file contents using [`tokio::fs::read_to_string`].
    ///
    /// The entire file is read into memory. For large files, consider
    /// using streaming APIs instead.
    async fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref();
        read_to_string(path).await.map_err(|e| Error::Io(e).into())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests with a mock filesystem implementation
}

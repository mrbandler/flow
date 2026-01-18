//! Default implementation of the [`Space`] trait.
//!
//! This module provides [`DefaultSpace`], the standard implementation of
//! the space trait used in production. It is generic over the filesystem
//! implementation to allow for dependency injection during testing.
//!
//! # Overview
//!
//! The [`DefaultSpace`] struct manages the lifecycle of a Flow space,
//! including:
//!
//! - Creating the directory structure (`.flow/`, `journal/`)
//! - Writing and reading space metadata (`space.json`)
//! - Managing the Loro CRDT document for collaboration
//!
//! # Directory Structure
//!
//! When a space is initialized, the following structure is created:
//!
//! ```text
//! my-space/
//! ├── .flow/
//! │   ├── space.json    # Space metadata
//! │   └── space.loro    # Loro CRDT document snapshot
//! └── journal/          # Directory for journal entries
//! ```
//!
//! # Examples
//!
//! ```ignore
//! use flow_core::filesystem::LocalFilesystem;
//! use flow_core::space::default::DefaultSpace;
//! use flow_core::space::traits::Space;
//!
//! // Initialize a new space
//! let fs = LocalFilesystem;
//! let space = DefaultSpace::init(fs, "./my-space", "personal").await?;
//! ```

use std::path::{Path, PathBuf};

use loro::LoroDoc;
use miette::{ensure, IntoDiagnostic, Result};

use crate::{
    errors::Error,
    filesystem::Filesystem,
    space::{traits::Space, Locator, Metadata},
};

/// The directory name where Flow stores space metadata.
///
/// This hidden directory contains all Flow-specific files, keeping
/// the user's content directory clean.
const FLOW_DIR: &str = ".flow";

/// The filename for space metadata (JSON format).
///
/// This file stores the space name, version, and other metadata
/// needed to identify and manage the space.
const METADATA_FILE: &str = "space.json";

/// The filename for the Loro CRDT document snapshot.
///
/// This file contains a binary snapshot of the Loro CRDT document,
/// which enables offline-first collaboration and conflict resolution.
const DOCUMENT_FILE: &str = "space.loro";

/// The directory name for journal entries.
///
/// Journal entries are stored as individual files in this directory,
/// organized by date or other criteria.
const JOURNAL_DIR: &str = "journal";

/// The default implementation of a Flow space.
///
/// `DefaultSpace` is generic over the filesystem implementation, allowing
/// different storage backends to be used. In production, this is typically
/// [`LocalFilesystem`](crate::filesystem::LocalFilesystem), while tests
/// can inject mock implementations.
///
/// # Type Parameters
///
/// * `F` - The filesystem implementation to use for all I/O operations.
///   Must implement [`Filesystem`] and be thread-safe (`Send + Sync`).
///
/// # Fields
///
/// * `fs` - The filesystem backend for all I/O operations.
/// * `metadata` - Space metadata including name and version.
/// * `doc` - The Loro CRDT document for collaborative editing.
///
/// # Thread Safety
///
/// `DefaultSpace` is `Send` and `Sync` when the filesystem implementation
/// is also `Send` and `Sync`, making it safe for use in async contexts.
///
/// # Examples
///
/// Initializing a new space:
///
/// ```ignore
/// use flow_core::filesystem::LocalFilesystem;
/// use flow_core::space::default::DefaultSpace;
/// use flow_core::space::traits::Space;
///
/// let fs = LocalFilesystem;
/// let space = DefaultSpace::init(fs, "./my-space", "personal").await?;
/// ```
///
/// Using a mock filesystem for testing:
///
/// ```ignore
/// use flow_core::space::default::DefaultSpace;
/// use flow_core::space::traits::Space;
///
/// let mock_fs = MockFilesystem::new();
/// let space = DefaultSpace::init(mock_fs, "./test-space", "test").await?;
/// ```
#[allow(dead_code)]
pub struct DefaultSpace<F: Filesystem> {
    /// The filesystem implementation used for all I/O operations.
    ///
    /// This is injected at construction time, allowing different backends
    /// to be used in production vs. testing environments.
    fs: F,

    /// Path of the space.
    path: PathBuf,

    /// Metadata of the space.
    ///
    /// Contains the space name, version, and other identifying information.
    /// This is persisted to `.flow/space.json`.
    metadata: Metadata,

    /// Loro CRDT document for offline and local-first collaboration.
    ///
    /// This document stores the space's content in a format that supports
    /// conflict-free merging, enabling offline editing and real-time
    /// collaboration.
    doc: LoroDoc,
}

impl<F: Filesystem> Space for DefaultSpace<F> {
    type Fs = F;

    /// Initializes a new space at the given path.
    ///
    /// This implementation performs the following steps:
    ///
    /// 1. Validates that the path exists and is an empty directory
    /// 2. Creates the `.flow/` directory for metadata storage
    /// 3. Creates the `journal/` directory for journal entries
    /// 4. Writes the space metadata to `.flow/space.json`
    /// 5. Creates and persists an empty Loro CRDT document
    ///
    /// # Implementation Notes
    ///
    /// - The space metadata is serialized as JSON using [`serde_json`].
    /// - The Loro document is exported as a binary snapshot for efficiency.
    /// - All filesystem operations use the injected `fs` implementation.
    async fn init(fs: Self::Fs, path: impl AsRef<Path> + Send + Sync, name: impl Into<String>) -> Result<Self>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let exists = fs.exists(path).await?;
        ensure!(exists, Error::NotFound(path.to_path_buf()));

        let is_dir = fs.is_dir(path).await?;
        ensure!(is_dir, Error::NotADirectory(path.to_path_buf()));

        let flow_dir = path.join(FLOW_DIR);
        let is_empty = fs.is_dir_empty(path).await?;
        if !is_empty {
            let has_space = fs.exists(&flow_dir).await?;
            if has_space {
                return Err(Error::AlreadyExists(path.to_path_buf()).into());
            }
            return Err(Error::DirectoryNotEmpty(path.to_path_buf()).into());
        }

        let journal_dir = path.join(JOURNAL_DIR);
        fs.create_dir(&flow_dir).await?;
        fs.create_dir(&journal_dir).await?;

        let metadata = Metadata {
            name: name.into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        let metadata_json = serde_json::to_string_pretty(&metadata).into_diagnostic()?;
        let metadata_path = flow_dir.join(METADATA_FILE);
        fs.write(&metadata_path, metadata_json.as_bytes()).await?;

        let doc = LoroDoc::new();
        let doc_snapshot = doc.export(loro::ExportMode::Snapshot).into_diagnostic()?;
        let doc_path = flow_dir.join(DOCUMENT_FILE);
        fs.write(&doc_path, &doc_snapshot).await?;

        Ok(Self {
            fs,
            path: path.to_path_buf(),
            metadata,
            doc,
        })
    }

    /// Loads an existing space from the given locator.
    ///
    /// # Implementation Notes
    ///
    /// This method will:
    ///
    /// 1. Resolve the [`Locator`] to a filesystem path
    /// 2. Read and deserialize the space metadata from `.flow/space.json`
    /// 3. Load the Loro CRDT document from `.flow/space.loro`
    ///
    /// # Unimplemented
    ///
    /// This method is not yet implemented and will panic if called.
    async fn load(fs: Self::Fs, locator: Locator) -> Result<Self>
    where
        Self: Sized,
    {
        let path = match locator {
            Locator::Name(_name) => todo!("Look up the path through the name of the space"),
            Locator::Path(path) => path,
        };

        let flow_dir = path.join(FLOW_DIR);
        let metadata_path = flow_dir.join(METADATA_FILE);
        let metadata_json = fs.read_to_string(&metadata_path).await?;
        let metadata = serde_json::from_str::<Metadata>(&metadata_json).into_diagnostic()?; // TODO: Create custom error for this?

        let doc_path = flow_dir.join(DOCUMENT_FILE);
        let doc_snapshot = fs.read(&doc_path).await?;
        let doc = LoroDoc::from_snapshot(&doc_snapshot).into_diagnostic()?; // TODO: Create custom error for this?

        Ok(Self {
            fs,
            path,
            metadata,
            doc,
        })
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests with a mock filesystem implementation
}

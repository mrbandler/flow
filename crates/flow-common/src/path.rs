//! Path manipulation utilities.
//!
//! This module provides extension traits for working with filesystem paths,
//! particularly for cross-platform path normalization.

use std::path::{Path, PathBuf};

/// Extension trait for [`PathBuf`] providing additional utility methods.
///
/// This trait adds methods to [`PathBuf`] that are useful across the Flow
/// codebase, particularly for displaying paths in a user-friendly format.
pub trait PathBufExt {
    /// Returns a normalized path.
    ///
    /// On Windows, this removes the extended-length path prefix (`\\?\`)
    /// that can appear when paths are canonicalized. On other platforms,
    /// this simply returns the path unchanged (zero allocation).
    #[must_use]
    fn normalize(self) -> Self;

    /// Returns a normalized path as a string.
    ///
    /// On Windows, this removes the extended-length path prefix (`\\?\`)
    /// that can appear when paths are canonicalized.
    fn normalize_to_string(self) -> String;
}

impl PathBufExt for PathBuf {
    fn normalize(self) -> Self {
        #[cfg(windows)]
        {
            Self::from(self.normalize_to_string())
        }

        #[cfg(not(windows))]
        self
    }

    fn normalize_to_string(self) -> String {
        let mut s = self.display().to_string();

        #[cfg(windows)]
        if s.starts_with(r"\\?\") {
            s.drain(..4);
        }

        s
    }
}

/// Extension trait for [`Path`] providing additional utility methods.
///
/// This trait adds methods to [`Path`] that are useful across the Flow
/// codebase, particularly for displaying paths in a user-friendly format.
pub trait PathExt {
    /// Returns a normalized path.
    ///
    /// On Windows, this removes the extended-length path prefix (`\\?\`)
    /// that can appear when paths are canonicalized. On other platforms,
    /// this returns a copy of the path.
    fn normalize(&self) -> PathBuf;

    fn normalize_to_string(&self) -> String;
}

impl PathExt for Path {
    fn normalize(&self) -> PathBuf {
        #[cfg(windows)]
        {
            PathBuf::from(self.normalize_to_string())
        }

        #[cfg(not(windows))]
        self.to_path_buf()
    }

    fn normalize_to_string(&self) -> String {
        let mut s = self.display().to_string();

        #[cfg(windows)]
        if s.starts_with(r"\\?\") {
            s.drain(..4);
        }

        s
    }
}

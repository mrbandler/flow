//! Path manipulation utilities.
//!
//! This module provides extension traits for working with filesystem paths,
//! particularly for cross-platform path normalization.

use std::path::Path;

/// Extension trait for [`Path`] providing additional utility methods.
///
/// This trait adds methods to [`Path`] that are useful across the Flow
/// codebase, particularly for displaying paths in a user-friendly format.
pub trait PathExt {
    /// Returns a normalized string representation of the path.
    ///
    /// On Windows, this removes the extended-length path prefix (`\\?\`)
    /// that can appear when paths are canonicalized. On other platforms,
    /// this simply returns the path's display string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use flow_common::PathExt;
    ///
    /// let path = Path::new("/home/user/notes");
    /// assert_eq!(path.normalize(), "/home/user/notes");
    /// ```
    fn normalize(&self) -> String;
}

impl PathExt for Path {
    fn normalize(&self) -> String {
        let s = self.display().to_string();

        #[cfg(windows)]
        {
            // Remove Windows extended-length path prefix
            if let Some(stripped) = s.strip_prefix(r"\\?\") {
                return stripped.to_string();
            }
        }

        s
    }
}

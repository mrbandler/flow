//! Internal trait definitions for space implementations.
//!
//! This module contains the internal traits that define the contract
//! for space implementations. These traits are not part of the public API.

use std::path::Path;

use miette::Result;

use crate::{filesystem::Filesystem, space::locator::Locator};

/// Internal trait defining the core operations for a space.
///
/// This trait is implemented by [`DefaultSpace`] and potentially other
/// space implementations. It is not exposed publicly; instead, the public
/// [`Space`](super::Space) struct wraps implementations of this trait.
///
/// # Design Notes
///
/// The trait is generic over the filesystem implementation (`Fs`), which
/// enables dependency injection for testing. Production code uses
/// [`LocalFilesystem`](crate::filesystem::LocalFilesystem), while tests
/// can provide mock implementations.
///
/// # Implementors
///
/// - [`DefaultSpace`](super::default::DefaultSpace) - The standard implementation.
///
/// # Examples
///
/// Implementing a custom space type:
///
/// ```ignore
/// use flow_core::filesystem::Filesystem;
/// use flow_core::space::{Locator, traits::Space};
/// use miette::Result;
/// use std::path::Path;
///
/// struct CustomSpace<F: Filesystem> {
///     fs: F,
///     // ... other fields
/// }
///
/// impl<F: Filesystem> Space for CustomSpace<F> {
///     type Fs = F;
///
///     async fn init(fs: Self::Fs, path: impl AsRef<Path>, name: &str) -> Result<Self> {
///         // Custom initialization logic
///         todo!()
///     }
///
///     async fn load(fs: Self::Fs, locator: Locator) -> Result<Self> {
///         // Custom loading logic
///         todo!()
///     }
/// }
/// ```
pub trait Space: Sized + Send + Sync {
    /// The filesystem implementation used by this space.
    ///
    /// This associated type allows space implementations to be generic
    /// over different filesystem backends. The filesystem must implement
    /// [`Filesystem`] and be thread-safe (`Send + Sync`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use flow_core::filesystem::LocalFilesystem;
    ///
    /// // In production, use LocalFilesystem
    /// type Fs = LocalFilesystem;
    ///
    /// // In tests, use a mock filesystem
    /// type Fs = MockFilesystem;
    /// ```
    type Fs: Filesystem + Send + Sync;

    /// Initialize a new space at the given path.
    ///
    /// This creates the necessary directory structure and configuration
    /// files for a new space. The space will be registered with the given
    /// name for future lookup.
    ///
    /// # Arguments
    ///
    /// * `fs` - The filesystem implementation to use.
    /// * `path` - The directory path where the space will be created.
    /// * `name` - A human-readable name for the space.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path already contains a space.
    /// - The directory cannot be created.
    /// - The configuration file cannot be written.
    async fn init(fs: Self::Fs, path: impl AsRef<Path> + Send + Sync, name: impl Into<String>) -> Result<Self>;

    /// Load an existing space from the given locator.
    ///
    /// The locator can specify either a space name (which is resolved
    /// from the global configuration) or an explicit filesystem path.
    ///
    /// # Arguments
    ///
    /// * `fs` - The filesystem implementation to use.
    /// * `locator` - Identifies which space to load.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The space cannot be found.
    /// - The space configuration is invalid or corrupted.
    /// - The filesystem cannot be read.
    async fn load(fs: Self::Fs, locator: Locator) -> Result<Self>;

    /// Returns the name of the space.
    ///
    /// The name is a human-readable identifier for the space, set during
    /// initialization. It is used for display purposes and can be used
    /// to look up the space in the registry.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let space = DefaultSpace::init(fs, "./my-space", "personal").await?;
    /// assert_eq!(space.name(), "personal");
    /// ```
    fn name(&self) -> &str;

    /// Returns the filesystem path to the space directory.
    ///
    /// This is the root directory containing the `.flow/` subdirectory
    /// and all space content.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let space = DefaultSpace::init(fs, "./my-space", "personal").await?;
    /// assert_eq!(space.path(), Path::new("./my-space"));
    /// ```
    fn path(&self) -> &Path;
}

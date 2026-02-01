use std::path::{Path, PathBuf};

use miette::Result;

/// An abstraction over filesystem operations.
///
/// This trait defines the minimal set of operations needed for Flow to
/// interact with a filesystem. Implementations must be thread-safe
/// (`Send + Sync`) to support async operations across threads.
///
/// # Implementors
///
/// - [`LocalFilesystem`] - Reads and writes to the local filesystem.
///
/// # Examples
///
/// Using the filesystem trait with dependency injection:
///
/// ```ignore
/// struct NoteStore<F: Filesystem> {
///     fs: F,
///     root: PathBuf,
/// }
///
/// impl<F: Filesystem> NoteStore<F> {
///     async fn save_note(&self, name: &str, content: &str) -> Result<()> {
///         let path = self.root.join(name).with_extension("md");
///         self.fs.write(&path, content).await
///     }
/// }
/// ```
#[allow(dead_code)]
pub trait Filesystem: Send + Sync {
    /// Checks if a path exists on the filesystem.
    ///
    /// Returns `true` if the path exists (file or directory), `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check for existence.
    ///
    /// # Errors
    ///
    /// Returns an error if the filesystem cannot be queried (e.g., permission denied).
    async fn exists(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool>;

    /// Checks if a path is a directory.
    ///
    /// Returns `true` if the path exists and is a directory, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be queried.
    async fn is_dir(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool>;

    /// Checks if a directory is empty.
    ///
    /// Returns `true` if the path is a directory with no entries, `false` otherwise.
    /// Returns `false` if the path is not a directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path to check.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read (e.g., permission denied).
    async fn is_dir_empty(&self, path: impl AsRef<Path> + Send + Sync) -> Result<bool>;

    /// Creates a new directory at the given path.
    ///
    /// The parent directory must already exist. This does not create parent directories.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the directory should be created.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent directory does not exist.
    /// - A file or directory already exists at the path.
    /// - Permission is denied.
    async fn create_dir(&self, path: impl AsRef<Path> + Send + Sync) -> Result<()>;

    /// Removes an empty directory at the given path.
    ///
    /// The directory must be empty; use [`remove_dir_all`](Self::remove_dir_all)
    /// to remove a directory and its contents.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory to remove.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The directory does not exist.
    /// - The directory is not empty.
    /// - Permission is denied.
    async fn remove_dir(&self, path: impl AsRef<Path> + Send + Sync) -> Result<()>;

    /// Creates a new directory at the given path.
    ///
    /// The parent directory must already exist. This does create parent directories.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the directory should be created.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent directory does not exist.
    /// - A file or directory already exists at the path.
    /// - Permission is denied.
    async fn create_dir_all(&self, path: impl AsRef<Path> + Send + Sync) -> Result<()>;

    /// Removes a directory and all of its contents recursively.
    ///
    /// This is a destructive operation that will delete everything within
    /// the directory. Use with caution.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory to remove.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist.
    /// - The path is not a directory.
    /// - Permission is denied.
    async fn remove_dir_all(&self, path: impl AsRef<Path> + Send + Sync) -> Result<()>;

    /// Writes content to a file at the given path.
    ///
    /// Creates the file if it doesn't exist, or overwrites it if it does.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to write.
    /// * `contents` - The byte content to write to the file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent directory does not exist.
    /// - Permission is denied.
    /// - The disk is full or another I/O error occurs.
    async fn write(&self, path: impl AsRef<Path> + Send + Sync, contents: impl AsRef<[u8]> + Send + Sync)
        -> Result<()>;

    /// Reads the entire contents of a file into a byte vector.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist or cannot be read.
    async fn read(&self, path: impl AsRef<Path> + Send + Sync) -> Result<Vec<u8>>;

    /// Reads the entire contents of a file into a string.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist or cannot be read.
    async fn read_to_string(&self, path: impl AsRef<Path> + Send + Sync) -> Result<String>;

    /// Returns the canonical, absolute form of a path.
    ///
    /// This resolves symbolic links and normalizes the path. On Windows,
    /// the returned path will have the `\\?\` prefix stripped.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to canonicalize.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or cannot be resolved.
    async fn canonicalize(&self, path: impl AsRef<Path> + Send + Sync) -> Result<PathBuf>;
}

//! Theme resolution from various sources.
//!
//! This module provides the [`resolve`] function that loads a [`Base16Palette`]
//! from a [`ThemeSource`] — whether it's a built-in name, a local file, or a URL.

use std::path::Path;

use flow_errors::ThemeError;

use crate::palette::Base16Palette;
use crate::source::ThemeSource;

impl ThemeSource {
    /// Resolves this [`ThemeSource`] to a [`Base16Palette`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The built-in theme name is not recognized
    /// - The file cannot be read
    /// - The URL cannot be fetched
    /// - The YAML is invalid or missing required fields
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flow_theme::ThemeSource;
    ///
    /// # async fn example() -> miette::Result<()> {
    /// let source: ThemeSource = "dracula".into();
    /// let palette = source.resolve().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resolve(&self) -> Result<Base16Palette, ThemeError> {
        match self {
            Self::Name(name) => Base16Palette::builtin(name),
            Self::File(path) => load_from_file(path).await,
            Self::Url(url) => fetch_from_url(url).await,
        }
    }
}

/// Loads a theme from a local YAML file.
async fn load_from_file(path: &Path) -> Result<Base16Palette, ThemeError> {
    // Expand ~ to home directory
    let expanded_path = if path.starts_with("~") {
        dirs_path().map_or_else(
            || path.to_path_buf(),
            |home| home.join(path.strip_prefix("~").unwrap_or(path)),
        )
    } else {
        path.to_path_buf()
    };

    let content = tokio::fs::read_to_string(&expanded_path)
        .await
        .map_err(|e| ThemeError::FileLoadFailed(format!("{}: {e}", expanded_path.display())))?;

    parse_yaml(&content)
}

/// Fetches a theme from a URL.
async fn fetch_from_url(url: &str) -> Result<Base16Palette, ThemeError> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| ThemeError::FetchFailed(format!("{url}: {e}")))?;

    if !response.status().is_success() {
        return Err(ThemeError::FetchFailed(format!("{url}: HTTP {}", response.status())));
    }

    let content = response
        .text()
        .await
        .map_err(|e| ThemeError::FetchFailed(format!("{url}: {e}")))?;

    parse_yaml(&content)
}

/// Parses YAML content into a `Base16Palette`.
fn parse_yaml(content: &str) -> Result<Base16Palette, ThemeError> {
    serde_yaml::from_str(content).map_err(|e| ThemeError::InvalidYaml(e.to_string()))
}

/// Gets the user's home directory.
fn dirs_path() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .ok()
            .map(std::path::PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(std::path::PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::ThemeSource;

    #[tokio::test]
    async fn test_resolve_builtin() {
        let source = ThemeSource::Name("flow".to_string());
        let palette = source.resolve().await.unwrap();
        assert!(palette.scheme.is_some());
    }

    #[tokio::test]
    async fn test_resolve_default_flow() {
        let source = ThemeSource::Name("flow".to_string());
        let palette = source.resolve().await.unwrap();
        assert_eq!(palette.scheme.as_deref(), Some("Flow"));
    }
}

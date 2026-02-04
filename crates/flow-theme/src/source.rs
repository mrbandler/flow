//! Theme source classification.
//!
//! The [`ThemeSource`] enum provides type-safe classification of theme strings
//! into built-in names, file paths, or URLs.

use std::path::PathBuf;
use std::str::FromStr;

/// Identifies the source of a theme.
///
/// A theme can come from a built-in name, a local YAML file, or a remote URL.
/// The source is determined automatically from a string via [`From<&str>`]:
///
/// - Starts with `http://` or `https://` → [`ThemeSource::Url`]
/// - Has `.yaml`/`.yml` extension, path separators, `~`, or `.` prefix → [`ThemeSource::File`]
/// - Everything else → [`ThemeSource::Name`]
///
/// # Examples
///
/// ```
/// use flow_theme::ThemeSource;
///
/// let source: ThemeSource = "dracula".into();
/// assert!(matches!(source, ThemeSource::Name(_)));
///
/// let source: ThemeSource = "~/themes/custom.yaml".into();
/// assert!(matches!(source, ThemeSource::File(_)));
///
/// let source: ThemeSource = "https://example.com/theme.yaml".into();
/// assert!(matches!(source, ThemeSource::Url(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeSource {
    /// A built-in theme name (e.g., "dracula", "nord").
    Name(String),
    /// A local YAML file path.
    File(PathBuf),
    /// A remote URL to a YAML theme file.
    Url(String),
}

impl From<&str> for ThemeSource {
    fn from(s: &str) -> Self {
        if s.starts_with("http://") || s.starts_with("https://") {
            return Self::Url(s.to_string());
        }

        if is_file_path(s) {
            return Self::File(PathBuf::from(s));
        }

        Self::Name(s.to_string())
    }
}

impl FromStr for ThemeSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

/// Determines if the theme string looks like a file path.
fn is_file_path(theme: &str) -> bool {
    let path = std::path::Path::new(theme);
    let has_yaml_extension = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml"));

    has_yaml_extension
        || theme.contains('/')
        || theme.contains('\\')
        || theme.starts_with('~')
        || theme.starts_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_source() {
        assert!(matches!(ThemeSource::from("dracula"), ThemeSource::Name(n) if n == "dracula"));
        assert!(matches!(ThemeSource::from("nord"), ThemeSource::Name(n) if n == "nord"));
        assert!(matches!(ThemeSource::from("flow"), ThemeSource::Name(n) if n == "flow"));
    }

    #[test]
    fn test_file_source() {
        assert!(matches!(
            ThemeSource::from("~/themes/my-theme.yaml"),
            ThemeSource::File(_)
        ));
        assert!(matches!(ThemeSource::from("./theme.yaml"), ThemeSource::File(_)));
        assert!(matches!(ThemeSource::from("/path/to/theme.yml"), ThemeSource::File(_)));
        assert!(matches!(
            ThemeSource::from("C:\\themes\\theme.yaml"),
            ThemeSource::File(_)
        ));
    }

    #[test]
    fn test_url_source() {
        assert!(matches!(
            ThemeSource::from("https://example.com/theme.yaml"),
            ThemeSource::Url(u) if u == "https://example.com/theme.yaml"
        ));
        assert!(matches!(
            ThemeSource::from("http://example.com/theme.yaml"),
            ThemeSource::Url(u) if u == "http://example.com/theme.yaml"
        ));
    }

    #[test]
    fn test_from_str() {
        let source: ThemeSource = "dracula".parse().unwrap();
        assert!(matches!(source, ThemeSource::Name(_)));
    }
}

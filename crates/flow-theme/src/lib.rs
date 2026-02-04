//! Theme system for Flow using base16 color schemes.
//!
//! This crate provides a configurable theming system based on the base16 color scheme
//! specification. Themes can be loaded from:
//!
//! - Built-in themes (e.g., "dracula", "nord", "gruvbox-dark")
//! - Local YAML files (e.g., "~/.config/flow/themes/my-theme.yaml")
//! - Remote URLs (e.g., `https://raw.githubusercontent.com/.../theme.yaml`)
//!
//! # Base16 Color Scheme
//!
//! Base16 defines 16 colors (base00-base0F) that are mapped semantically:
//!
//! - **base00-base07**: Shades from darkest to lightest (backgrounds and foregrounds)
//! - **base08**: Red (errors, deletions)
//! - **base09**: Orange (integers, constants)
//! - **base0A**: Yellow (warnings, classes)
//! - **base0B**: Green (success, strings)
//! - **base0C**: Cyan (info, support)
//! - **base0D**: Blue (primary, functions)
//! - **base0E**: Magenta (keywords)
//! - **base0F**: Brown (deprecated, embedded)
//!
//! # Examples
//!
//! ```no_run
//! use flow_theme::{ThemeSource, Base16Palette};
//!
//! # async fn example() -> miette::Result<()> {
//! // Load a built-in theme
//! let source: ThemeSource = "dracula".into();
//! let palette = source.resolve().await?;
//!
//! // Load from a file
//! let source: ThemeSource = "~/.config/flow/themes/custom.yaml".into();
//! let palette = source.resolve().await?;
//!
//! // Load from a URL
//! let source: ThemeSource = "https://example.com/theme.yaml".into();
//! let palette = source.resolve().await?;
//! # Ok(())
//! # }
//! ```

mod builtin;
mod palette;
mod resolve;
mod source;
mod theme;

pub use palette::{Base16Palette, HexColor};
pub use source::ThemeSource;
pub use theme::Theme;

//! Built-in base16 color schemes.
//!
//! This module provides a collection of embedded color schemes that can be
//! used without requiring external files or network access.

use flow_errors::ThemeError;

use super::{Base16Palette, HexColor};

/// Gets a built-in theme by name.
///
/// # Errors
///
/// Returns `ThemeError::UnknownBuiltIn` if the theme name is not recognized.
pub fn get(name: &str) -> Result<Base16Palette, ThemeError> {
    match name.to_lowercase().as_str() {
        "flow" => Ok(flow()),
        "dracula" => Ok(dracula()),
        "nord" => Ok(nord()),
        "gruvbox-dark" | "gruvbox" => Ok(gruvbox_dark()),
        "solarized-dark" | "solarized" => Ok(solarized_dark()),
        "catppuccin-mocha" | "catppuccin" => Ok(catppuccin_mocha()),
        "tokyo-night" | "tokyonight" => Ok(tokyo_night()),
        "one-dark" | "onedark" => Ok(one_dark()),
        "monokai" => Ok(monokai()),
        "rose-pine" | "rosepine" => Ok(rose_pine()),
        _ => Err(ThemeError::UnknownBuiltIn(name.to_string())),
    }
}

/// Returns a list of all available built-in theme names.
#[must_use]
pub fn list() -> Vec<&'static str> {
    vec![
        "flow",
        "dracula",
        "nord",
        "gruvbox-dark",
        "solarized-dark",
        "catppuccin-mocha",
        "tokyo-night",
        "one-dark",
        "monokai",
        "rose-pine",
    ]
}

/// Flow's default theme - based on the original cyan/green/red CLI colors.
#[must_use]
pub fn flow() -> Base16Palette {
    Base16Palette {
        scheme: Some("Flow".to_string()),
        author: Some("Flow Contributors".to_string()),
        base00: HexColor::new("1a1b26"), // Dark background
        base01: HexColor::new("24283b"), // Lighter background
        base02: HexColor::new("414868"), // Selection
        base03: HexColor::new("565f89"), // Comments (dim)
        base04: HexColor::new("9aa5ce"), // Dark foreground
        base05: HexColor::new("c0caf5"), // Default foreground
        base06: HexColor::new("d5d6db"), // Light foreground
        base07: HexColor::new("e0e0e0"), // Lightest foreground
        base08: HexColor::new("f7768e"), // Red (error)
        base09: HexColor::new("ff9e64"), // Orange
        base0a: HexColor::new("e0af68"), // Yellow (warning)
        base0b: HexColor::new("9ece6a"), // Green (success)
        base0c: HexColor::new("7dcfff"), // Cyan (info/step)
        base0d: HexColor::new("7aa2f7"), // Blue (primary/prompt)
        base0e: HexColor::new("bb9af7"), // Magenta
        base0f: HexColor::new("c0caf5"), // Brown/deprecated
    }
}

/// Dracula theme - popular dark theme with purple and pink accents.
#[must_use]
pub fn dracula() -> Base16Palette {
    Base16Palette {
        scheme: Some("Dracula".to_string()),
        author: Some("Zeno Rocha".to_string()),
        base00: HexColor::new("282a36"), // Background
        base01: HexColor::new("3a3c4e"), // Current Line
        base02: HexColor::new("44475a"), // Selection
        base03: HexColor::new("6272a4"), // Comment
        base04: HexColor::new("8be9fd"), // Cyan
        base05: HexColor::new("f8f8f2"), // Foreground
        base06: HexColor::new("f1fa8c"), // Yellow
        base07: HexColor::new("ffffff"), // White
        base08: HexColor::new("ff5555"), // Red
        base09: HexColor::new("ffb86c"), // Orange
        base0a: HexColor::new("f1fa8c"), // Yellow
        base0b: HexColor::new("50fa7b"), // Green
        base0c: HexColor::new("8be9fd"), // Cyan
        base0d: HexColor::new("bd93f9"), // Purple
        base0e: HexColor::new("ff79c6"), // Pink
        base0f: HexColor::new("ff5555"), // Red
    }
}

/// Nord theme - arctic, north-bluish color palette.
#[must_use]
pub fn nord() -> Base16Palette {
    Base16Palette {
        scheme: Some("Nord".to_string()),
        author: Some("Arctic Ice Studio".to_string()),
        base00: HexColor::new("2e3440"), // Polar Night
        base01: HexColor::new("3b4252"),
        base02: HexColor::new("434c5e"),
        base03: HexColor::new("4c566a"),
        base04: HexColor::new("d8dee9"), // Snow Storm
        base05: HexColor::new("e5e9f0"),
        base06: HexColor::new("eceff4"),
        base07: HexColor::new("8fbcbb"), // Frost
        base08: HexColor::new("bf616a"), // Aurora Red
        base09: HexColor::new("d08770"), // Aurora Orange
        base0a: HexColor::new("ebcb8b"), // Aurora Yellow
        base0b: HexColor::new("a3be8c"), // Aurora Green
        base0c: HexColor::new("88c0d0"), // Frost
        base0d: HexColor::new("81a1c1"), // Frost
        base0e: HexColor::new("b48ead"), // Aurora Purple
        base0f: HexColor::new("5e81ac"), // Frost
    }
}

/// Gruvbox Dark theme - retro groove color scheme.
#[must_use]
pub fn gruvbox_dark() -> Base16Palette {
    Base16Palette {
        scheme: Some("Gruvbox Dark".to_string()),
        author: Some("morhetz".to_string()),
        base00: HexColor::new("282828"), // Hard dark
        base01: HexColor::new("3c3836"),
        base02: HexColor::new("504945"),
        base03: HexColor::new("665c54"),
        base04: HexColor::new("bdae93"),
        base05: HexColor::new("d5c4a1"),
        base06: HexColor::new("ebdbb2"),
        base07: HexColor::new("fbf1c7"),
        base08: HexColor::new("fb4934"), // Red
        base09: HexColor::new("fe8019"), // Orange
        base0a: HexColor::new("fabd2f"), // Yellow
        base0b: HexColor::new("b8bb26"), // Green
        base0c: HexColor::new("8ec07c"), // Aqua
        base0d: HexColor::new("83a598"), // Blue
        base0e: HexColor::new("d3869b"), // Purple
        base0f: HexColor::new("d65d0e"), // Brown
    }
}

/// Solarized Dark theme - precision colors for machines and people.
#[must_use]
pub fn solarized_dark() -> Base16Palette {
    Base16Palette {
        scheme: Some("Solarized Dark".to_string()),
        author: Some("Ethan Schoonover".to_string()),
        base00: HexColor::new("002b36"), // Base03
        base01: HexColor::new("073642"), // Base02
        base02: HexColor::new("586e75"), // Base01
        base03: HexColor::new("657b83"), // Base00
        base04: HexColor::new("839496"), // Base0
        base05: HexColor::new("93a1a1"), // Base1
        base06: HexColor::new("eee8d5"), // Base2
        base07: HexColor::new("fdf6e3"), // Base3
        base08: HexColor::new("dc322f"), // Red
        base09: HexColor::new("cb4b16"), // Orange
        base0a: HexColor::new("b58900"), // Yellow
        base0b: HexColor::new("859900"), // Green
        base0c: HexColor::new("2aa198"), // Cyan
        base0d: HexColor::new("268bd2"), // Blue
        base0e: HexColor::new("6c71c4"), // Violet
        base0f: HexColor::new("d33682"), // Magenta
    }
}

/// Catppuccin Mocha - soothing pastel theme.
#[must_use]
pub fn catppuccin_mocha() -> Base16Palette {
    Base16Palette {
        scheme: Some("Catppuccin Mocha".to_string()),
        author: Some("Catppuccin".to_string()),
        base00: HexColor::new("1e1e2e"), // Base
        base01: HexColor::new("181825"), // Mantle
        base02: HexColor::new("313244"), // Surface0
        base03: HexColor::new("45475a"), // Surface1
        base04: HexColor::new("585b70"), // Surface2
        base05: HexColor::new("cdd6f4"), // Text
        base06: HexColor::new("f5e0dc"), // Rosewater
        base07: HexColor::new("b4befe"), // Lavender
        base08: HexColor::new("f38ba8"), // Red
        base09: HexColor::new("fab387"), // Peach
        base0a: HexColor::new("f9e2af"), // Yellow
        base0b: HexColor::new("a6e3a1"), // Green
        base0c: HexColor::new("94e2d5"), // Teal
        base0d: HexColor::new("89b4fa"), // Blue
        base0e: HexColor::new("cba6f7"), // Mauve
        base0f: HexColor::new("f2cdcd"), // Flamingo
    }
}

/// Tokyo Night theme - clean, dark theme inspired by Tokyo city lights.
#[must_use]
pub fn tokyo_night() -> Base16Palette {
    Base16Palette {
        scheme: Some("Tokyo Night".to_string()),
        author: Some("enkia".to_string()),
        base00: HexColor::new("1a1b26"), // Background
        base01: HexColor::new("16161e"), // Terminal black
        base02: HexColor::new("2f3549"), // Selection
        base03: HexColor::new("444b6a"), // Comment
        base04: HexColor::new("787c99"), // Dark foreground
        base05: HexColor::new("a9b1d6"), // Foreground
        base06: HexColor::new("cbccd1"), // Light foreground
        base07: HexColor::new("d5d6db"), // White
        base08: HexColor::new("f7768e"), // Red
        base09: HexColor::new("ff9e64"), // Orange
        base0a: HexColor::new("e0af68"), // Yellow
        base0b: HexColor::new("9ece6a"), // Green
        base0c: HexColor::new("7dcfff"), // Cyan
        base0d: HexColor::new("7aa2f7"), // Blue
        base0e: HexColor::new("bb9af7"), // Magenta
        base0f: HexColor::new("c0caf5"), // Brown
    }
}

/// One Dark theme - Atom's iconic dark theme.
#[must_use]
pub fn one_dark() -> Base16Palette {
    Base16Palette {
        scheme: Some("One Dark".to_string()),
        author: Some("Atom".to_string()),
        base00: HexColor::new("282c34"), // Background
        base01: HexColor::new("353b45"), // Lighter background
        base02: HexColor::new("3e4451"), // Selection
        base03: HexColor::new("545862"), // Comment
        base04: HexColor::new("565c64"), // Dark foreground
        base05: HexColor::new("abb2bf"), // Foreground
        base06: HexColor::new("b6bdca"), // Light foreground
        base07: HexColor::new("c8ccd4"), // White
        base08: HexColor::new("e06c75"), // Red
        base09: HexColor::new("d19a66"), // Orange
        base0a: HexColor::new("e5c07b"), // Yellow
        base0b: HexColor::new("98c379"), // Green
        base0c: HexColor::new("56b6c2"), // Cyan
        base0d: HexColor::new("61afef"), // Blue
        base0e: HexColor::new("c678dd"), // Magenta
        base0f: HexColor::new("be5046"), // Brown
    }
}

/// Monokai theme - the classic Sublime Text theme.
#[must_use]
pub fn monokai() -> Base16Palette {
    Base16Palette {
        scheme: Some("Monokai".to_string()),
        author: Some("Wimer Hazenberg".to_string()),
        base00: HexColor::new("272822"), // Background
        base01: HexColor::new("383830"), // Lighter background
        base02: HexColor::new("49483e"), // Selection
        base03: HexColor::new("75715e"), // Comment
        base04: HexColor::new("a59f85"), // Dark foreground
        base05: HexColor::new("f8f8f2"), // Foreground
        base06: HexColor::new("f5f4f1"), // Light foreground
        base07: HexColor::new("f9f8f5"), // White
        base08: HexColor::new("f92672"), // Red/Pink
        base09: HexColor::new("fd971f"), // Orange
        base0a: HexColor::new("f4bf75"), // Yellow
        base0b: HexColor::new("a6e22e"), // Green
        base0c: HexColor::new("a1efe4"), // Cyan
        base0d: HexColor::new("66d9ef"), // Blue
        base0e: HexColor::new("ae81ff"), // Purple
        base0f: HexColor::new("cc6633"), // Brown
    }
}

/// Rosé Pine theme - all natural pine, faux fur and a bit of soho vibes.
#[must_use]
pub fn rose_pine() -> Base16Palette {
    Base16Palette {
        scheme: Some("Rosé Pine".to_string()),
        author: Some("Rosé Pine".to_string()),
        base00: HexColor::new("191724"), // Base
        base01: HexColor::new("1f1d2e"), // Surface
        base02: HexColor::new("26233a"), // Overlay
        base03: HexColor::new("6e6a86"), // Muted
        base04: HexColor::new("908caa"), // Subtle
        base05: HexColor::new("e0def4"), // Text
        base06: HexColor::new("e0def4"), // Text
        base07: HexColor::new("e0def4"), // Text
        base08: HexColor::new("eb6f92"), // Love (red)
        base09: HexColor::new("f6c177"), // Gold (orange)
        base0a: HexColor::new("f6c177"), // Gold (yellow)
        base0b: HexColor::new("31748f"), // Pine (green)
        base0c: HexColor::new("9ccfd8"), // Foam (cyan)
        base0d: HexColor::new("c4a7e7"), // Iris (blue/purple)
        base0e: HexColor::new("ebbcba"), // Rose (magenta)
        base0f: HexColor::new("524f67"), // Highlight med
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_builtin_themes() {
        for name in list() {
            assert!(get(name).is_ok(), "Failed to get theme: {name}");
        }
    }

    #[test]
    fn test_get_unknown_theme() {
        assert!(matches!(get("nonexistent"), Err(ThemeError::UnknownBuiltIn(_))));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(get("DRACULA").is_ok());
        assert!(get("Dracula").is_ok());
        assert!(get("dracula").is_ok());
    }

    #[test]
    fn test_aliases() {
        assert!(get("gruvbox").is_ok());
        assert!(get("gruvbox-dark").is_ok());
        assert!(get("catppuccin").is_ok());
        assert!(get("catppuccin-mocha").is_ok());
    }
}

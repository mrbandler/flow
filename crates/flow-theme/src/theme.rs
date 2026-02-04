use crate::{Base16Palette, HexColor};

/// Theme trait providing access to Base16 color palettes and semantic colors.
pub trait Theme {
    /// Returns the Base16 color palette.
    #[must_use]
    fn palette(&self) -> &Base16Palette;

    /// Returns the success color (green, base0B).
    #[must_use]
    fn success(&self) -> &HexColor {
        &self.palette().base0b
    }

    /// Returns the error color (red, base08).
    #[must_use]
    fn error(&self) -> &HexColor {
        &self.palette().base08
    }

    /// Returns the warning color (yellow, base0A).
    #[must_use]
    fn warning(&self) -> &HexColor {
        &self.palette().base0a
    }

    /// Returns the info color (cyan, base0C).
    #[must_use]
    fn info(&self) -> &HexColor {
        &self.palette().base0c
    }

    /// Returns the primary/prompt color (blue, base0D).
    #[must_use]
    fn primary(&self) -> &HexColor {
        &self.palette().base0d
    }

    /// Returns the highlight/selection color (orange, base09).
    #[must_use]
    fn highlight(&self) -> &HexColor {
        &self.palette().base09
    }

    /// Returns the dim/comment color (base03).
    #[must_use]
    fn dim(&self) -> &HexColor {
        &self.palette().base03
    }

    /// Returns the foreground color (base05).
    #[must_use]
    fn foreground(&self) -> &HexColor {
        &self.palette().base05
    }
}

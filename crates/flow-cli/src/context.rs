//! Application context for CLI commands.
//!
//! The [`Context`] struct bundles shared resources that commands need access to,
//! such as configuration and theming. It is initialized once at CLI startup and
//! passed to each command.

use flow_core::Config;
use flow_theme::ThemeSource;
use miette::Result;

use crate::theme::CliTheme;

/// Application context containing shared resources for CLI commands.
///
/// The context is created once at startup and passed to each command,
/// providing access to:
///
/// - **Configuration**: User settings and registered spaces
/// - **Theme**: Colors and styling for terminal output
///
/// # Example
///
/// ```ignore
/// let mut ctx = Context::load().await?;
/// let theme = ctx.theme();
/// let config = ctx.config_mut();
/// ```
pub struct Context {
    config: Config,
    theme: CliTheme,
}

impl Context {
    /// Loads the application context.
    ///
    /// This initializes all shared resources:
    /// 1. Loads configuration from disk
    /// 2. Resolves and initializes the theme from config
    /// 3. Registers the theme globally for inquire prompts
    ///
    /// # Errors
    ///
    /// Returns an error if configuration loading or theme resolution fails.
    pub async fn load() -> Result<Self> {
        let config = Config::load().await?;

        let source: ThemeSource = config.settings().theme.as_deref().unwrap_or("flow").into();
        let palette = source.resolve().await?;
        let theme = CliTheme::new(palette);
        theme.register();

        Ok(Self { config, theme })
    }

    /// Returns a reference to the configuration.
    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// Returns a mutable reference to the configuration.
    pub const fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Returns a reference to the theme.
    #[must_use]
    pub const fn theme(&self) -> &CliTheme {
        &self.theme
    }
}

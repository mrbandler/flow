# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Configuration module for managing Flow settings and registered spaces
  - `Config` struct with methods to register, unregister, and find spaces
  - Support for setting and clearing the active space
  - Persistent storage in `~/.config/flow/` (config.json and spaces.json)
  - Theme configuration field with support for builtin base16 palettes, base16 YAML files, and remote URLs
- `flow-theme` crate for shared theming logic and base16 data types across frontends
- Unified inquire theme with consistent prompt styling and shared symbols
  - Centralized symbols in `theme::symbols` (prompt `?` in cyan, success `✓` in green, error `✗` in red, selection arrow `→` in cyan)
- Space management CLI commands
  - `flow space init` - Initialize a new space at a path (with path validation)
  - `flow space list` - List all registered spaces with active marker
  - `flow space switch` - Change the active space
  - `flow space register` - Register an existing space (with path validation)
  - `flow space unregister` - Remove a space from the registry (with optional `--delete`)
- CLI context object that passes loaded configuration and theme at startup
- `Locator` type for identifying spaces by name or filesystem path
- `Printer` module for consistent CLI output with support for JSON, verbose, and quiet modes
- Path normalization utilities in `flow-common` for cross-platform compatibility
- `Command` trait for consistent CLI command lifecycle (init, validate, interactive, execute, finalize)
- Global CLI arguments: `--interactive`, `--json`, `--verbose`, `--quiet`, `--trace`
- Stdin piping support for pre-filling missing command arguments from piped input
- Structured logging via `tracing` with CLI printer integration (library logs routed through `--verbose`/`--trace`)
- Custom path serializer to output paths with forward slashes on all platforms
- Space module with trait-based abstraction pattern for easy testing and extensibility
- Filesystem module with local filesystem implementation
- Initial project structure with workspace layout
- CLI foundation with clap argument parsing
- TUI crate placeholder (feature-gated)
- GUI crate placeholder (feature-gated)
- Server crate placeholder (feature-gated)
- Core library crate for shared functionality
- App crate for shared application logic
- Pre-commit hooks configuration
- CI workflow with formatting, linting, testing, and security checks
- Release workflow with multi-platform binary builds
- GitHub issue templates (bug report, feature request, documentation)
- Pull request template
- Comprehensive contributing guidelines

### Changed

- Replaced `console` crate with `crossterm` for terminal handling (consistent with inquire)
- Replaced emoji symbols with clean Unicode symbols in CLI output
- Simplified space loading by parsing `Locator` directly from arguments (removed `load_space` from `SpaceArgs`)
- `flow space init` no longer forces interactive mode when only the name argument is missing (name is derived from the directory path)

### Deprecated

- Nothing yet

### Removed

- Nothing yet

### Fixed

- Paths stored in configuration are now properly canonicalized and normalized
- Windows extended-length path prefix (`\\?\`) is stripped from displayed paths
- Fixed `unregister` doc example to include `delete` parameter
- `flow space init` now rejects already-registered space names before initializing

### Security

- Nothing yet

<!--
## [0.1.0] - YYYY-MM-DD

### Added

- Feature description here

### Changed

- Change description here

### Fixed

- Fix description here
-->

[Unreleased]: https://github.com/mrbandler/flow/compare/HEAD...HEAD
<!-- [0.1.0]: https://github.com/mrbandler/flow/releases/tag/v0.1.0 -->

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
- Space management CLI commands
  - `flow space init` - Initialize a new space at a path
  - `flow space list` - List all registered spaces with active marker
  - `flow space switch` - Change the active space
  - `flow space register` - Register an existing space
  - `flow space unregister` - Remove a space from the registry (with optional `--delete`)
- `Locator` type for identifying spaces by name or filesystem path
- `Printer` module for consistent CLI output with support for JSON, verbose, and quiet modes
- Path normalization utilities in `flow-common` for cross-platform compatibility
- `Command` trait for consistent CLI command lifecycle (init, validate, interactive, execute, finalize)
- Global CLI arguments: `--interactive`, `--json`, `--verbose`, `--quiet`
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

- Nothing yet

### Deprecated

- Nothing yet

### Removed

- Nothing yet

### Fixed

- Paths stored in configuration are now properly canonicalized and normalized
- Windows extended-length path prefix (`\\?\`) is stripped from displayed paths

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

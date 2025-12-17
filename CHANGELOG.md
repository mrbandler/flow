# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure with workspace organization
- Core library (`flow-core`) with configuration and space management
- CLI crate (`flow-cli`) with commands:
  - `init` - Initialize a new Flow graph
  - `open` - Open an existing graph
  - `add` - Add content to today's journal
  - `clean` - Remove orphaned graphs from configuration
- TUI crate (`flow-tui`) - Terminal user interface (placeholder)
- Desktop crate (`flow-desktop`) - Desktop application using iced (placeholder)
- App crate (`flow-app`) - Shared application logic
- Feature flags for binary variants:
  - Default: CLI only
  - `tui`: CLI + Terminal UI
  - `desktop`: CLI + Desktop application
  - `all`: Full binary with all features
- CI/CD pipeline with GitHub Actions:
  - Formatting checks (rustfmt)
  - Linting (clippy)
  - Security audits (cargo-deny)
  - Cross-platform testing (Linux, macOS, Windows)
  - Documentation builds
  - Code coverage reporting
  - Automated releases with cross-compilation
- Development tooling configuration:
  - `rustfmt.toml` - Code formatting rules
  - `clippy.toml` - Lint configuration
  - `deny.toml` - Dependency auditing
  - `rust-toolchain.toml` - Rust version pinning
- Documentation:
  - `README.md` - Project overview and philosophy
  - `CONTRIBUTING.md` - Contribution guidelines
  - `CHANGELOG.md` - Version history

### Changed

- Nothing yet

### Deprecated

- Nothing yet

### Removed

- Nothing yet

### Fixed

- Nothing yet

### Security

- Nothing yet

## [0.1.0] - Unreleased

Initial release (planned).

### Planned Features

- Basic CLI with capture and list commands
- Local markdown storage with outliner support
- Simple search functionality
- Configuration management with XDG-style paths

---

[Unreleased]: https://github.com/mrbandler/flow/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mrbandler/flow/releases/tag/v0.1.0
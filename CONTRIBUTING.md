# Contributing to Flow

Thank you for your interest in contributing to Flow! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- Git
- A code editor (we recommend VS Code with rust-analyzer)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:

   ```bash
   git clone https://github.com/YOUR_USERNAME/flow.git
   cd flow
   ```

3. Add the upstream repository:

   ```bash
   git remote add upstream https://github.com/mrbandler/flow.git
   ```

## Development Setup

### Install Rust Toolchain

The project includes a `rust-toolchain.toml` file that specifies the required Rust version. Simply run:

```bash
rustup show
```

This will automatically install the correct toolchain with required components.

### Install Development Tools

```bash
# Code formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Dependency auditing
cargo install cargo-deny

# Changelog generation (optional)
cargo install git-cliff

# Documentation site (optional)
cargo install mdbook

# Watch mode for development (optional)
cargo install cargo-watch
```

### Install Pre-commit Hooks

We use [pre-commit](https://pre-commit.com/) to automatically run checks before commits. This ensures code quality and consistency across all contributions.

```bash
# Install pre-commit (choose one method)
pip install pre-commit          # via pip
pipx install pre-commit         # via pipx (recommended)
brew install pre-commit         # via Homebrew (macOS)
scoop install pre-commit        # via Scoop (Windows)
```

Then install the git hooks:

```bash
# Install pre-commit hooks
pre-commit install

# Install commit message validation (for conventional commits)
pre-commit install --hook-type commit-msg

# Install pre-push hooks (runs tests and cargo-deny)
pre-commit install --hook-type pre-push
```

The hooks will now run automatically on every commit. To run all hooks manually:

```bash
pre-commit run --all-files
```

### Verify Setup

```bash
# Check that everything compiles
cargo build --workspace --all-features

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run lints
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Project Structure

```
flow/
├── crates/
│   ├── flow/              # Main binary crate
│   │   └── src/
│   │       └── main.rs    # Entry point with feature flags
│   ├── flow-core/         # Core library (data structures, storage)
│   ├── flow-app/          # Application logic (shared between UIs)
│   ├── flow-cli/          # CLI commands and interface
│   ├── flow-tui/          # Terminal UI (ratatui, crossterm)
│   ├── flow-gui/          # Desktop GUI (iced)
│   ├── flow-server/       # Server (future)
├── tests/                 # Integration tests
├── Cargo.toml             # Workspace configuration
├── rustfmt.toml           # Formatting rules
├── clippy.toml            # Lint configuration
├── deny.toml              # Dependency audit rules
└── rust-toolchain.toml    # Rust version pinning
```

### Binary Variants

Flow builds a single fat binary with varying features using feature flags:

| Features  | Description                |
|-----------|----------------------------|
| (default) | CLI only                   |
| `tui`     | CLI + Terminal UI          |
| `gui`     | CLI + Desktop application  |
| `server`  | CLI + Server               |
| `all`     | Everything included        |

Build specific variants:

```bash
# CLI only (default)
cargo build --package flow

# CLI + TUI
cargo build --package flow --features tui

# CLI + Desktop
cargo build --package flow --features gui

# CLI + Server
cargo build --package flow --features server

# Full binary
cargo build --package flow --features all
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test additions/changes

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting (no code change)
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:

```
feat(cli): add interactive mode for init command
fix(core): handle empty graph paths correctly
docs(readme): update installation instructions
refactor(app): extract graph loading into separate module
test(cli): add tests for add command
```

## Coding Standards

### Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt --all
```

Configuration is in `rustfmt.toml`.

### Linting

All code must pass `clippy` without warnings:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Configuration is in `clippy.toml` and `Cargo.toml` workspace lints.

### Documentation

- All public items must have documentation comments
- Use `///` for item documentation
- Use `//!` for module-level documentation
- Include examples in doc comments where appropriate
- Run `cargo doc --workspace --all-features` to verify docs build

### Error Handling

- Use `miette` for user-facing errors with helpful messages
- Use `thiserror` for library error types
- Provide context with `.context()` or `.with_context()`
- Never use `.unwrap()` in library code (okay in tests)

### Code Style

- Prefer `impl Trait` over `dyn Trait` where possible
- Use `#[must_use]` for functions with important return values
- Prefer iterators over explicit loops
- Keep functions focused and under 100 lines
- Use meaningful variable names

## Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test --package flow-core

# Specific test
cargo test --package flow-cli test_name

# With output
cargo test --workspace -- --nocapture
```

### Writing Tests

- Place unit tests in a `#[cfg(test)] mod tests` block
- Place integration tests in `tests/` directory
- Use descriptive test names: `test_<function>_<scenario>_<expected>`
- Test both success and error cases
- Use `proptest` for property-based testing where appropriate

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_creates_default_when_missing() {
        // Arrange
        let temp_dir = tempfile::tempdir().unwrap();

        // Act
        let config = Config::load_from(temp_dir.path()).unwrap();

        // Assert
        assert!(config.spaces.is_empty());
        assert!(config.active_space.is_none());
    }
}
```

## Submitting Changes

### Before Submitting

1. Ensure all checks pass:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo deny check
   ```

2. Update documentation if needed
3. Add tests for new functionality
4. Update CHANGELOG.md (if applicable)

### Pull Request Process

1. Update your branch with the latest upstream changes:

   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. Push your branch and create a pull request

3. Fill out the PR template with:
   - Description of changes
   - Related issues
   - Testing performed
   - Screenshots (for UI changes)

4. Wait for CI to pass

5. Address review feedback

6. Once approved, a maintainer will merge your PR

### Pull Request Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted with `rustfmt`
- [ ] Code passes `clippy` lints
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (for notable changes)
- [ ] Commit messages follow conventions

## Release Process

Releases are automated via GitHub Actions when a version tag is pushed.

### Version Bumping

1. Update version in all `Cargo.toml` files (workspace manages this)
2. Update `CHANGELOG.md` with release notes
3. Commit: `git commit -am "chore: bump version to x.y.z"`
4. Tag: `git tag vx.y.z`
5. Push: `git push && git push --tags`

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

Pre-release versions use suffixes: `v0.1.0-alpha.1`, `v0.1.0-beta.1`, `v0.1.0-rc.1`

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues and discussions first

Thank you for contributing to Flow! 🚀

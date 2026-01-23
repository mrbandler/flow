# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Flow is a local-first, outliner-based note-taking tool for developers. It stores data in plain Markdown files (the source of truth) with CRDT-based sync capabilities. The project is in early development.

## Build Commands

```bash
# Build workspace
cargo build --workspace --all-features

# Build specific variant
cargo build --package flow                    # CLI only (default)
cargo build --package flow --features tui     # CLI + TUI
cargo build --package flow --features gui     # CLI + GUI
cargo build --package flow --features server  # CLI + Server
cargo build --package flow --features all     # Everything

# Run tests
cargo test --workspace
cargo test --package flow-core              # Specific crate
cargo test --package flow-cli test_name     # Specific test

# Linting and formatting
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

## Architecture

### Crate Structure

- **flow** - Main binary with feature flags for different modes (CLI/TUI/GUI/Server)
- **flow-core** - Core library: Space management, filesystem abstraction, configuration
- **flow-app** - Application logic layer (placeholder, coordinates between core and frontends)
- **flow-cli** - CLI commands using clap, supports interactive and JSON output modes
- **flow-errors** - Centralized error types using miette for rich diagnostics
- **flow-common** - Shared utilities (e.g., path normalization)
- **flow-tui** - Terminal UI (ratatui)
- **flow-gui** - Desktop GUI (iced)
- **flow-server** - Sync server

### Data Flow

```
Markdown Files (Source of Truth)
  ↓ import/export
CRDT Storage (Persistence & Sync via Loro)
  ↓ parse
FlowNode AST (Custom syntax: properties, tags, references)
  ↓ convert
In-Memory Graph (nodes keyed by ID, indices for fast lookup)
  ↓ index
Full-Text Search Engine
```

### Node Identity System

- **Temporary IDs** (`t:abc123`): Position-based, regenerated each session
- **Stable IDs** (`n:abc123`): Permanent, embedded as HTML comments in markdown (`<!-- n:abc123 -->`)
- Nodes are promoted from temp to stable when first tagged, referenced, or given properties

### CLI Command Pattern

Commands implement the `Command` trait for consistent:

- Interactive prompting when arguments are missing
- Execution logic
- Output formatting (human-readable or JSON via `--json`)

## Code Conventions

### Error Handling

- Use `miette` for user-facing errors with helpful messages
- Use `thiserror` for library error types
- Never use `.unwrap()` in library code (allowed in tests)
- Errors live in `flow-errors` crate

### Workspace Lints

Defined in root `Cargo.toml`:

- `unsafe_code = "forbid"`
- Clippy: `all`, `pedantic`, `nursery` at warn level

### Commit Messages

Follow Conventional Commits: `type(scope): description`

- Types: feat, fix, docs, style, refactor, test, chore

## Key Design Decisions

- **Markdown as source of truth** - All data in human-readable markdown, git-friendly
- **CRDT for sync** - Loro CRDT enables offline-first, conflict-free sync
- **Tree-based outliner** - Not a graph database; uses efficient lookups and explicit indices
- **Local-first** - All features work offline, sync is optional
- **Single fat binary** - Feature flags control which interfaces are included

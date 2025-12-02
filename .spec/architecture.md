# Flow System Architecture

## Overview

Flow is a privacy-focused, local-first note-taking system. The architecture is designed around the principle that **markdown files are the absolute source of truth**, with all other components being derived or supporting layers.

This document describes the high-level system architecture and how the major components interact.

---

## System Components

```
┌──────────────────────────────────────────────────────────────────┐
│                         User Interfaces                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │   CLI    │  │   TUI    │  │   GUI    │  │   Web Interface  │ │
│  │ (clap)   │  │(ratatui) │  │  (iced)  │  │                  │ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────┬─────────┘ │
└───────┼─────────────┼─────────────┼─────────────────┼───────────┘
        │             │             │                 │
        └─────────────┴──────┬──────┴─────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────────┐
│                        Core Module                               │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                     Public API                             │ │
│  │  Space Management • Node Operations • Queries • Validation │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌──────────────┐ ┌──────────────┐ ┌────────────────────────┐  │
│  │ Graph State  │ │ Query Engine │ │   Validation System    │  │
│  │              │ │              │ │                        │  │
│  │ • Nodes      │ │ • SQL-like   │ │ • Reference integrity  │  │
│  │ • Tags       │ │   queries    │ │ • Schema validation    │  │
│  │ • Properties │ │ • Full-text  │ │ • Type checking        │  │
│  │ • References │ │   search     │ │                        │  │
│  └──────┬───────┘ └──────┬───────┘ └────────────┬───────────┘  │
│         │                │                      │               │
│  ┌──────▼────────────────▼──────────────────────▼─────────────┐ │
│  │                   Index System                             │ │
│  │   by_file • by_tag • backlinks • full-text search          │ │
│  └────────────────────────────┬───────────────────────────────┘ │
│                               │                                  │
│  ┌────────────────────────────▼───────────────────────────────┐ │
│  │                  Persistence Layer                         │ │
│  │   Markdown Parser • CRDT Storage • File Watcher • Export   │ │
│  └────────────────────────────┬───────────────────────────────┘ │
└───────────────────────────────┼──────────────────────────────────┘
                                │
┌───────────────────────────────▼──────────────────────────────────┐
│                        Filesystem                                │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────────────┐ │
│  │  journals/  │  │   pages/    │  │        .flow/            │ │
│  │  (daily     │  │  (named     │  │  • space.loro (CRDT)     │ │
│  │   notes)    │  │   pages)    │  │  • search_index/         │ │
│  └─────────────┘  └─────────────┘  │  • config.toml           │ │
│                                     └──────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
                                │
                                │ (optional)
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                        Sync Server                               │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    HTTP/WebSocket API                      │ │
│  └────────────────────────────────────────────────────────────┘ │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐  │
│  │  CRDT Merge      │  │  Device Registry │  │  Auth        │  │
│  │  Engine          │  │                  │  │  (optional)  │  │
│  └──────────────────┘  └──────────────────┘  └──────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Component Descriptions

### User Interfaces

Flow provides multiple interfaces to accommodate different user preferences and use cases:

| Interface | Description | Use Case |
|-----------|-------------|----------|
| **CLI** | Command-line interface | Scripting, automation, power users |
| **TUI** | Terminal user interface | Keyboard-driven editing in terminal |
| **GUI** | Native desktop application | General desktop use |
| **Web** | Browser-based interface | Access from any device |

All interfaces communicate exclusively through the Core Module's public API, ensuring consistent behavior regardless of how users interact with their data.

### Core Module

The Core Module is the heart of Flow. It provides all functionality for managing spaces, nodes, queries, and persistence.

**Key Responsibilities:**
- Node lifecycle management (create, read, update, delete)
- Tag and property management
- Reference tracking and backlink computation
- Query execution (SQL-like queries and full-text search)
- Markdown parsing and rendering
- CRDT operations for sync
- Index maintenance for fast lookups
- Validation and integrity checking

**Public API Surface:**
- Space management (open, close, create)
- Node operations (add, edit, delete, move)
- Tag operations (apply, remove, query)
- Property operations (set, get, delete)
- Reference operations (resolve, backlinks, mentions)
- Query operations (SQL-like, full-text, tag-based)
- Validation (check, fix, restore)

### Sync Server

The Sync Server is an **optional** component that enables multi-device synchronization.

**Key Responsibilities:**
- Accept CRDT operations from connected devices
- Merge operations and broadcast to other devices
- Track device registration and last-sync state
- Optional authentication for shared spaces

**Design Principles:**
- Self-hostable (users control their data)
- Stateless operation merging (CRDT handles conflicts)
- No access to decrypted content (future: end-to-end encryption)

---

## Data Flow

### Local Edit Flow

```
User Action → UI Layer → Core API → Graph State → Render → CRDT Update → File Export
                                                                    ↓
                                                            (if sync enabled)
                                                                    ↓
                                                              Sync Server
                                                                    ↓
                                                              Other Devices
```

### External Edit Flow

```
External Editor → File System → File Watcher → CRDT Update → Re-parse → Graph Update
                                                                    ↓
                                                            (if sync enabled)
                                                                    ↓
                                                              Sync Server
```

### Sync Flow

```
Device A: Edit → CRDT Op → Send to Server
                                ↓
Server: Receive → Merge → Broadcast
                                ↓
Device B: Receive → CRDT Apply → Re-parse → Graph Update → Export
```

---

## Storage Architecture

### Space Directory Structure

```
my-space/
├── .flow/
│   ├── space.loro          # CRDT document (sync state)
│   ├── search_index/       # Full-text search index
│   └── config.toml         # Space configuration
├── journals/
│   ├── 2024-11-30.md       # Daily notes
│   └── 2024-11-29.md
└── pages/
    ├── projects/
    │   └── flow.md         # Named pages
    └── meetings/
        └── standup.md
```

### Global Configuration

```
~/.config/flow/
├── config.toml             # Global settings
└── spaces.toml             # Space registry
```

---

## Key Design Decisions

### Markdown as Source of Truth

- All data is stored in human-readable markdown files
- Files can be edited with any text editor
- Git-friendly for version control
- No proprietary formats or lock-in

### CRDT for Sync

- Conflict-free replicated data types enable offline-first operation
- Automatic merge without manual conflict resolution
- Character-level merge for concurrent edits
- Full history preserved for recovery

### Hybrid ID System

- **Temporary IDs** (`t:abc123`): Position-based, regenerated each session
- **Stable IDs** (`n:abc123`): Permanent, stored as HTML comments in markdown
- Promotion happens automatically when nodes get tags, properties, or references

### Local-First Philosophy

- All features work offline
- Sync is optional, not required
- User data stays on user's devices
- Self-hosted sync server option

---

## Interface Boundaries

### Core Module API

The Core Module exposes a well-defined API that all frontends use:

```
Space Operations:
  - open(path) → Space
  - close(space)
  - create(path, name) → Space

Node Operations:
  - add(content, options) → Node
  - edit(node_id, content)
  - delete(node_id)
  - move(node_id, new_parent)
  - show(node_id) → Node

Query Operations:
  - query(sql) → [Node]
  - find(text) → [Node]
  - tagged(tag) → [Node]
  - refs(node_id) → [Node]
  - backlinks(node_id) → [Node]

Tag Operations:
  - tag(node_id, tags)
  - untag(node_id, tags)

Property Operations:
  - prop_set(node_id, key, value)
  - prop_get(node_id, key) → Value
  - prop_delete(node_id, key)

Validation Operations:
  - check() → ValidationResult
  - fix(issues)
  - restore(timestamp)
```

### Sync Protocol

Communication between clients and sync server:

```
Client → Server:
  - CRDT operations (text deltas)
  - Device registration
  - Sync state queries

Server → Client:
  - Merged CRDT operations
  - Peer updates
  - Sync confirmations
```

---

## Performance Targets

| Operation | Target |
|-----------|--------|
| Startup (1000 files) | < 1 second |
| Node creation | < 10ms |
| Tag query | < 1ms |
| Full-text search | < 10ms |
| Sync latency | < 100ms end-to-end |

---

## Security Model

### Local Security
- All data stored locally by default
- Standard filesystem permissions
- No elevation required

### Sync Security (when enabled)
- Self-hosted option for full control
- Future: End-to-end encryption
- No telemetry or analytics

---

## Future Considerations

### Plugin System
- Custom syntax extensions
- Custom property types
- Query language extensions

### Additional Imports
- Obsidian vault import
- Roam JSON import
- Standard markdown export

### Advanced Sync
- Selective sync by path/tag
- Bandwidth optimization
- Compression
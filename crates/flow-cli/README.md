# flow-cli

Command-line interface for Flow, a local-first outliner-based note-taking tool.

## Design Principles

- **Markdown as storage** - Pages are markdown files. Users chose Flow for this; don't hide it.
- **Everything is a node** - Content is manipulated as nodes. Pages are the storage boundary.
- **Context-aware** - Commands operate on current space, page, and focused node.
- **Unix composable** - Commands accept stdin, output to stdout, chainable with pipes.
- **Explicit errors** - Fail loudly rather than silently do nothing.

## Core Entities

| Entity | What it is | Storage |
|--------|------------|---------|
| **Space** | Workspace containing pages | Directory with `.flow/` |
| **Page** | Container for nodes | Markdown file |
| **Node** | Content item (bullet) | Lines within markdown |

## Node Addressing

Nodes can be referenced in multiple ways:

| Format | Example | Description |
|--------|---------|-------------|
| Node ID | `n:abc123` | Globally unique identifier |
| Page path | `projects/flow` | References page root node |
| Full path | `projects/flow/n:abc123` | Explicit node within page |

All formats are interchangeable where a node reference is expected.

## Context State

Stored in `.flow/` directory:

- **Current space** - Active workspace
- **Current page** - Defaults to today's journal
- **Focused node** - Optional, narrows scope for node commands

The journal page for the current day always "exists" - Flow creates it on first access.

---

## Commands

### `flow space`

Manage workspaces.

| Command | Description |
|---------|-------------|
| `space init [path]` | Initialize new space at path (or current directory) |
| `space list` | List all known/registered spaces |
| `space switch <name\|path>` | Set active space |
| `space status` | Show current space info (path, page count, etc.) |
| `space register <name\|path>` | Registers space |
| `space unregister <name\|path>` | Unregister space (optionally delete files) |

### `flow page`

Manage markdown files (storage layer).

| Command | Description |
|---------|-------------|
| `page create <path>` | Create new page (supports `/` for directories) |
| `page list` | List all pages in current space |
| `page open <path>` | Set current page context |
| `page rename <old> <new>` | Rename a page |
| `page delete <path>` | Delete a page |

### `flow node`

Manipulate content (node layer).

#### Viewing

| Command | Description |
|---------|-------------|
| `node tree [id\|path]` | Show tree from node/page (focused node if omitted) |

#### Creating & Editing

| Command | Description |
|---------|-------------|
| `node add [content]` | Add node to current focus or page root |
| `node edit <id> [content]` | Edit node content |
| `node delete <id>` | Remove node |

Content input methods:

- Inline: `flow node add "My thought"`
- Editor: `flow node add` (opens `$EDITOR`)
- Interactive: Prompted when arguments missing
- Stdin: `echo "My thought" | flow node add`

#### Reordering (within siblings)

| Command | Description |
|---------|-------------|
| `node up <id> [count]` | Move up among siblings (default: 1) |
| `node down <id> [count]` | Move down among siblings (default: 1) |

#### Hierarchy Changes

| Command | Description |
|---------|-------------|
| `node indent <id>` | Nest under previous sibling |
| `node outdent <id>` | Unnest to parent's level |
| `node move <id> <parent>` | Reparent to specific node or page root |

#### Focus

| Command | Description |
|---------|-------------|
| `node focus <id>` | Set working context |
| `node unfocus` | Clear focus, return to page root |

---

## Error Behavior

| Situation | Result |
|-----------|--------|
| `node move n:x n:x` | Error: cannot move node to itself |
| `node indent` on first sibling | Error: no previous sibling to nest under |
| `node outdent` on root-level | Error: node is already at root level |
| `node up/down` exceeds range | Error: cannot move beyond bounds (n positions available) |
| `node delete <page-root>` | Equivalent to `page delete` (same storage) |
| `node add` with no journal | Auto-create today's journal, then add |

---

## Usage Examples

### Daily Capture

```bash
# Capture thoughts to today's journal (default)
flow node add "Remember to review PR #42"
flow node add "Idea: batch processing for imports"

# See today's entries
flow node tree
```

### Organize Within a Page

```bash
# View structure
flow node tree
# ├── n:a1 Remember to review PR #42
# ├── n:a2 Idea: batch processing
# └── n:a3 Meeting notes

# Create a parent and nest
flow node add "Ideas"
flow node move n:a2 n:a4

# Reorder to top
flow node up n:a4 3
```

### Working Across Pages

```bash
# Create project page
flow page create projects/flow

# Move node from journal to project
flow node move n:a2 projects/flow

# Peek without switching context
flow node tree projects/flow

# Or switch to work there
flow page open projects/flow
flow node add "Architecture decisions"
```

### Focus Mode

```bash
flow node focus n:b1

# Commands now scoped to this subtree
flow node add "Sub-item"
flow node tree  # Only shows focused subtree

flow node unfocus
```

### Unix Piping

```bash
# Search and view
flow search "query" | flow node tree

# Search and move
flow search "CLI" | flow node move - projects/flow

# Interactive selection
flow page list | fzf | flow node tree
```

---

## Global Flags

| Flag | Description |
|------|-------------|
| `--json` | Output as JSON (for scripting) |
| `--help` | Show help for command |
| `--version` | Show version |

---

## Future Considerations

Commands not in scope for core, but designed to fit:

- `flow search <query>` - Full-text search across nodes
- `flow tag <id> <tag>` - Add tag to node
- `flow property <id> <key> <value>` - Set node property
- `flow sync` - CRDT sync operations
- `flow export <id> [format]` - Export subtree

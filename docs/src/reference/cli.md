# CLI Reference

This chapter provides a complete reference for all Flow CLI commands.

> **Note:** Flow is in early development. Commands and options may change.

## Global Options

These options can be used with any command:

| Option | Short | Description |
|--------|-------|-------------|
| `--space <name>` | `-s` | Space to use (registered name or path) |
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version information |

## Commands

### `flow capture`

Quickly capture a thought without leaving your terminal.

```bash
flow capture "Your thought here #tag"
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<TEXT>` | The text to capture |

**Examples:**

```bash
# Simple capture
flow capture "Remember to update the docs"

# Capture with tags
flow capture "Research CRDT sync options #idea #research"

# Capture to a specific space
flow -s work capture "Meeting notes from standup"
```

---

### `flow search`

Search your knowledge base.

```bash
flow search <QUERY>
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<QUERY>` | Search query |

**Examples:**

```bash
# Simple search
flow search "project ideas"

# Search with tags
flow search "#rust #async"
```

---

### `flow list`

List notes in the current space.

```bash
flow list [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--limit <N>` | Maximum number of results |
| `--tag <TAG>` | Filter by tag |

---

### `flow init`

Initialize a new Flow space.

```bash
flow init [PATH]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `[PATH]` | Directory to initialize (defaults to current directory) |

**Examples:**

```bash
# Initialize in current directory
flow init

# Initialize in a specific directory
flow init ~/notes/personal
```

---

## UI Commands

These commands launch interactive interfaces (requires respective features to be compiled in).

### `flow tui`

Launch the Terminal User Interface.

```bash
flow tui
```

> Requires the `tui` feature.

### `flow gui`

Launch the Desktop GUI application.

```bash
flow gui
```

> Requires the `gui` feature.

### `flow serve`

Start the Flow server for sync and web access.

```bash
flow serve [OPTIONS]
```

> Requires the `server` feature.

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `FLOW_HOME` | Directory for Flow configuration and data |
| `FLOW_DEFAULT_SPACE` | Default space to use when `--space` is not specified |

---

## Exit Codes

| Code | Description |
|------|-------------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |

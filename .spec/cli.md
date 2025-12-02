# Flow CLI Command Specification

## Design Principles

1. **Dual Mode Operation**: Commands without required node IDs enter interactive mode
2. **Context Persistence**: Last accessed node stored as `.` reference
3. **Date References**: Journal nodes addressable by date expressions
4. **Inline Syntax**: Tags (#tag) and properties (key:: value) parsed from content strings
5. **Machine Parsable**: JSON output via `--json` flag
6. **Unix Philosophy**: Commands chainable via pipes, scriptable

## Global Flags

Available for all commands:

```
--json              Output in JSON format
--space <path>      Target specific space (overrides active)
--verbose, -v       Detailed logging
--quiet, -q         Suppress non-error output
--help, -h          Command help
```

## Context System

Context (`.`) refers to the last accessed node in the current session. Persisted in space's `.flow/context` file.

```
.                   Current context node
..                  Parent of current context
```

## Date Reference System

Journal nodes can be referenced by date expressions:

```
@today              Today's daily note
@yesterday          Yesterday's daily note  
@tomorrow           Tomorrow's daily note
@2024-11-24         Specific date (ISO 8601)
@-3d                3 days ago
@+1w                1 week from now
@-2m                2 months ago
```

Supported units: `d` (days), `w` (weeks), `m` (months), `y` (years)

## Inline Syntax for Tags and Properties

Content can include inline syntax for tags and properties, which are parsed and applied automatically.

### Tag Syntax

Tags use `#tag-name` format. Multiple tags allowed:

```bash
flow add "Project Alpha #project #active #q4"
# Content: "Project Alpha"
# Tags: [project, active, q4]

flow add "Meeting with team #meeting #important"
# Content: "Meeting with team"
# Tags: [meeting, important]
```

### Property Syntax

Properties use `key:: value` format (note the space after `::`):

```bash
flow add "Bug fix status:: in-progress priority:: high"
# Content: "Bug fix"
# Properties: {status: "in-progress", priority: "high"}

flow add "Research date:: 2024-11-25 owner:: michael"
# Content: "Research"
# Properties: {date: "2024-11-25", owner: "michael"}
```

### Multi-word Values

Quote values containing spaces:

```bash
flow add "Task status:: \"needs review\" assignee:: \"Alice Smith\""
# Properties: {status: "needs review", assignee: "Alice Smith"}
```

### Combined Syntax

Tags and properties can be mixed:

```bash
flow add "Design Review #meeting date:: 2024-11-25 attendees:: 5 #important"
# Content: "Design Review"
# Tags: [meeting, important]
# Properties: {date: "2024-11-25", attendees: "5"}
```

### Property Type Detection

Property values are automatically typed:

```
priority:: 5              → number
due:: 2024-11-25         → date
status:: done            → string
tags:: [ui,ux,design]    → list
owner:: ((n:abc123))     → reference
completed:: true         → boolean
```

### Parsing Rules

1. Extract all `#tag` patterns and apply as tags
2. Extract all `key:: value` patterns and apply as properties
3. Remove tag and property syntax from final node content
4. Tag and property syntax can appear anywhere in content string

### Autocomplete in Interactive Mode

When typing content in editor or interactive prompt, autocomplete activates:

**Tag Autocomplete:**
```
#proj<TAB>
  → #project
  
#mee<TAB>
  → #meeting
```

Suggests existing tags in space, frequency-ranked.

**Property Key Autocomplete:**
```
status::<TAB>
  → Suggests: status::active, status::done, status::blocked, status::in-progress
  
owner::<TAB>
  → Suggests existing owners: owner::michael, owner::alice
```

Shows existing keys and common values for each key.

**Property Value Autocomplete:**
```
status::<TAB>
  → Shows: active, done, blocked, in-progress (context-aware for this key)
  
priority::<TAB>
  → Shows: 1, 2, 3, high, medium, low
```

Autocomplete sources:
- Existing tags in space
- Property keys used in space  
- Property values for each key (context-aware)
- Frequency-ranked (most used suggestions first)

### Explicit Flags vs Inline Syntax

Inline syntax is the primary method. Explicit flags remain available for scripting:

```bash
# Inline (preferred for interactive use)
flow add "Task #urgent priority::high"

# Explicit flags (preferred for scripting)
flow add "Task" --tags urgent --property priority=high

# Mixed usage (flags override inline)
flow add "Task #work" --tags urgent,important
# Results in tags: [urgent, important] (inline #work ignored)
```

---

## Space Management Commands

### `flow init <path>`

Initialize a new Flow space.

**Arguments:**
- `<path>`: Directory path for new space

**Flags:**
- `--name <name>`: Space name (defaults to directory name)
- `--template <template>`: Initialize with template structure

**Behavior:**
- Creates directory structure
- Initializes Loro container
- Creates journal directory
- Writes space metadata

**Output:**
```
Initialized Flow space at /path/to/space
Space ID: n:a3k9m2
```

**Exit Codes:**
- 0: Success
- 1: Path already exists
- 2: Permission denied

---

### `flow open <path|name>`

Set the active space for subsequent commands.

**Arguments:**
- `<path|name>`: Path to existing space or registered space name

**Flags:**
- `--set-default`: Make this the default space

**Behavior:**
- Accepts either full/relative path or space name
- If name provided, looks up in `~/.config/flow/spaces.toml`
- If path provided, opens directly
- Validates space directory
- Updates `~/.config/flow/active_space`
- Loads space metadata

**Examples:**
```bash
flow open /home/user/notes          # Absolute path
flow open ~/work/flow-space         # Relative path
flow open personal                  # Registered name
flow open work                      # Registered name
```

**Output:**
```
Opened space: SpaceName
Path: /path/to/space
```

**Exit Codes:**
- 0: Success
- 1: Space not found (name lookup failed or invalid path)
- 2: Space corrupted

---

### `flow list`

List all registered spaces.

**Flags:**
- `--verbose, -v`: Show space statistics

**Output (default):**
```
* personal (/home/user/notes) [active]
  work (/home/user/work/flow)
  archive (/mnt/archive/old-notes)
```

**Output (--json):**
```json
{
  "spaces": [
    {
      "name": "personal",
      "path": "/home/user/notes",
      "active": true,
      "node_count": 1247,
      "last_modified": "2024-11-24T10:30:00Z"
    }
  ]
}
```

---

### `flow clean`

Remove orphaned spaces from configuration.

**Description:**
Scans all registered spaces in the configuration and removes entries for spaces whose directories no longer exist or are no longer valid Flow spaces.

**Flags:**
- `--dry-run`: Show what would be removed without making changes

**Behavior:**
- Checks each registered space path
- Verifies the space directory exists
- Verifies the `.flow` directory exists within it
- Removes invalid entries from configuration
- If active space is removed, clears active space selection

**Output (default):**
```
Checking 3 registered spaces...
Removed: work (/home/user/deleted-space) - directory not found
Removed: archive (/mnt/removed/notes) - not a valid space
Kept: personal (/home/user/notes)

Cleaned 2 orphaned spaces from configuration
```

**Output (--dry-run):**
```
Checking 3 registered spaces...
Would remove: work (/home/user/deleted-space) - directory not found
Would remove: archive (/mnt/removed/notes) - not a valid space
Would keep: personal (/home/user/notes)

Dry run: 2 spaces would be removed
```

**Output (--json):**
```json
{
  "checked": 3,
  "removed": [
    {
      "name": "work",
      "path": "/home/user/deleted-space",
      "reason": "directory not found"
    },
    {
      "name": "archive",
      "path": "/mnt/removed/notes",
      "reason": "not a valid space"
    }
  ],
  "kept": [
    {
      "name": "personal",
      "path": "/home/user/notes"
    }
  ]
}
```

**Exit Codes:**
- 0: Success (graphs removed or none needed removal)
- 1: Configuration error

---

### `flow status`

Show current space health and statistics.

**Flags:**
- `--check-integrity`: Validate CRDT state
- `--show-conflicts`: List unresolved conflicts

**Output:**
```
Space: SpaceName
Path: /path/to/space
Status: Clean

Nodes: 1,247
Tags: 23
References: 3,891
Modified: 2024-11-24 10:30:00

Dirty containers: 0
Uncommitted changes: 0
```

---

## Node Operations

### `flow add [content]`

Create node in today's journal.

**Arguments:**
- `[content]`: Node content with optional inline tags/properties (optional, opens $EDITOR if omitted)

**Flags:**
- `--date <date>`: Target specific date (default: @today)
- `--parent <node-id>`: Add as child of specific node
- `--tags <tags>`: Explicit tags (comma-separated, overrides inline)
- `--property <key>=<value>`: Explicit property (repeatable, overrides inline)
- `--editor, -e`: Force editor even with content

**Behavior (Explicit):**
```bash
flow add "Meeting notes #meeting status:: done"
# Creates node with content "Meeting notes", tag [meeting], property {status: "done"}

flow add "Task #urgent #important priority:: high"
# Tags: [urgent, important], property {priority: "high"}

flow add --date @yesterday "Forgot to log"
# Adds to yesterday's journal

# Explicit flags (scripting)
flow add "Task" --tags urgent,important --property priority=high
```

**Behavior (Interactive):**
```bash
flow add
# Opens $EDITOR with autocomplete, creates node on save
```

**Output:**
```
Created node: t:a3f821 (or n:abc123 if promoted)
In: journals/2024-11-24.md
Tags: [meeting]
Properties: {status: "done"}
```

---

### `flow edit [node-id]`

Edit node content.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--editor <editor>`: Override $EDITOR
- `--in-place`: Edit raw markdown file directly

**Behavior (Explicit):**
```bash
flow edit n:abc123
# Opens node in $EDITOR

flow edit @today
# Edits today's journal

flow edit .
# Edits current context node
```

**Behavior (Interactive):**
```bash
flow edit
# Enters search mode:
# 
# Search: _
# 
# [Fuzzy search results update as you type]
# [Enter to select, ESC to cancel]
```

**Interactive Search UI:**
```
Search: project cli_

  1. Project: Flow CLI
     tags: [project] modified: 2h ago
     └─ Implementing core commands...
     
  2. CLI Architecture Design  
     tags: [design] modified: 1d ago
     └─ Command structure and patterns...

[↑↓ Navigate | Enter Select | ESC Cancel]
```

**Output:**
```
Editing node: n:abc123
```

---

### `flow show [node-id]`

Display node content and metadata.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--with-children, -c`: Include child nodes
- `--with-refs, -r`: Show references
- `--raw`: Output raw markdown
- `--format <format>`: Output format (markdown, plain, tree)

**Behavior (Explicit):**
```bash
flow show n:abc123
flow show @today
flow show .
```

**Behavior (Interactive):**
```bash
flow show
# Enters search mode (same as edit)
```

**Output (default):**
```
Node: n:abc123
Created: 2024-11-20 14:22:00
Modified: 2024-11-24 10:15:00
Tags: [project, active]

# Project: Flow CLI

Implementing the CLI interface for Flow.

Properties:
  status:: in-progress
  priority:: high

References:
  → n:x1y2z3 (CLI Architecture Design)
  ← n:d4e5f6 (Implementation Tracker)

Children: 3 nodes
```

**Output (--raw):**
```
# Project: Flow CLI

Implementing the CLI interface for Flow.
```

---

### `flow delete [node-id]`

Delete node and optionally its children.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--recursive, -r`: Delete children too
- `--force, -f`: Skip confirmation
- `--keep-children`: Reparent children to deleted node's parent

**Behavior (Explicit):**
```bash
flow delete n:abc123
# Prompts: "Delete node 'Project: Flow CLI'? [y/N]"

flow delete n:abc123 --recursive --force
# Deletes without confirmation
```

**Behavior (Interactive):**
```bash
flow delete
# Search → select → confirm deletion
```

**Output:**
```
Deleted node: n:abc123
Reparented 3 children to parent node
```

---

### `flow move [node-id] [new-parent]`

Change node's parent.

**Arguments:**
- `[node-id]`: Node to move (optional)
- `[new-parent]`: New parent node (optional)

**Flags:**
- `--position <n>`: Insert at specific child position

**Behavior (Explicit):**
```bash
flow move n:abc123 n:xyz789
# Moves n:abc123 under n:xyz789

flow move n:abc123 @today
# Moves node to today's journal
```

**Behavior (Interactive):**
```bash
flow move
# Search for node → search for parent

flow move abc-123-def
# Search for parent only
```

**Output:**
```
Moved node: n:abc123
New parent: n:xyz789
```

---

### `flow append [node-id] [content]`

Add child node to parent.

**Arguments:**
- `[node-id]`: Parent node (optional)
- `[content]`: Child content with optional inline tags/properties (optional)

**Flags:**
- `--position <n>`: Insert at specific position
- `--tags <tags>`: Explicit tags (comma-separated, overrides inline)
- `--property <key>=<value>`: Explicit property (repeatable, overrides inline)

**Behavior (Explicit):**
```bash
flow append n:abc123 "Subtask #task status:: todo"
# Creates child under n:abc123 with tag [task] and property {status: "todo"}

flow append @today "Quick note #reminder"
# Adds to today's journal with tag [reminder]

# Explicit flags (scripting)
flow append n:abc123 "Subtask" --tags task,urgent --property status=todo
```

**Behavior (Interactive):**
```bash
flow append
# Search for parent → editor for content with autocomplete

flow append abc-123-def
# Opens editor for content
```

---

## Node Navigation

### `flow tree [node-id]`

Display node hierarchy.

**Arguments:**
- `[node-id]`: Root node (optional, defaults to current context)

**Flags:**
- `--depth <n>`: Limit tree depth
- `--tags`: Show tags inline
- `--ids`: Show node IDs

**Behavior (Explicit):**
```bash
flow tree n:abc123
flow tree @today
flow tree .
```

**Behavior (Interactive):**
```bash
flow tree
# Search for root node
```

**Output:**
```
Project: Flow CLI [project, active]
├── Core Architecture
│   ├── Space module
│   └── CRDT integration
├── CLI Implementation [in-progress]
│   ├── Command parsing
│   └── Interactive mode
└── Testing Strategy
    └── Integration tests
```

**Output (--json):**
```json
{
  "root": {
    "id": "n:abc123",
    "content": "Project: Flow CLI",
    "tags": ["project", "active"],
    "children": [...]
  }
}
```

---

### `flow children [node-id]`

List direct children of node.

**Arguments:**
- `[node-id]`: Parent node (optional)

**Flags:**
- `--count`: Show count only
- `--tags <tag>`: Filter by tag

**Behavior (Explicit):**
```bash
flow children n:abc123
flow children @today
```

**Behavior (Interactive):**
```bash
flow children
# Search for parent
```

**Output:**
```
Children of: Project: Flow CLI

  n:a1b2c3  Core Architecture
  n:d4e5f6  CLI Implementation [in-progress]
  n:g7h8i9  Testing Strategy

3 children
```

---

### `flow parent [node-id]`

Show parent of node.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
None

**Behavior (Explicit):**
```bash
flow parent n:abc123
flow parent .
```

**Behavior (Interactive):**
```bash
flow parent
# Search for node
```

**Output:**
```
Parent of: CLI Implementation

n:xyz789  Project: Flow CLI
```

---

### `flow path [node-id]`

Show breadcrumb trail from root to node.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--ids`: Show node IDs

**Behavior (Explicit):**
```bash
flow path n:d4e5f6
```

**Output:**
```
journals/2024-11-20.md
  → Project: Flow CLI
    → CLI Implementation
```

---

## References

References are created by embedding `((node-id))` syntax directly in node content. There are no explicit link/unlink commands - simply edit the node content to add or remove references.

### `flow refs [node-id]`

Show all references to and from node.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--incoming`: Show only backlinks
- `--outgoing`: Show only forward links
- `--count`: Show counts only

**Behavior (Explicit):**
```bash
flow refs n:abc123
flow refs .
```

**Behavior (Interactive):**
```bash
flow refs
# Search for node
```

**Output:**
```
References for: Project: Flow CLI

Outgoing (3):
  → n:x1y2z3  CLI Architecture Design
  → n:a4b5c6  Implementation Tracker
  → n:d7e8f9  Testing Guidelines

Incoming (5):
  ← n:q1w2e3  Q4 Projects
  ← n:r4t5y6  Active Development
  ← n:u7i8o9  Code Review Notes
  ← n:p0a1s2  Sprint Planning
  ← n:d3f4g5  Team Updates
```

---

### `flow backlinks [node-id]`

Show incoming references only (nodes that link to this node).

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--context, -c`: Show surrounding content

**Behavior (Explicit):**
```bash
flow backlinks n:abc123
flow backlinks .
```

**Behavior (Interactive):**
```bash
flow backlinks
# Search for node
```

**Output:**
```
Backlinks for: Project: Flow CLI

Incoming (5):
  ← n:q1w2e3  Q4 Projects
  ← n:r4t5y6  Active Development
  ← n:u7i8o9  Code Review Notes
  ← n:p0a1s2  Sprint Planning
  ← n:d3f4g5  Team Updates
```

---

### `flow mentions [node-id]`

Show outgoing references only (nodes that this node links to).

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--context, -c`: Show surrounding content

**Behavior (Explicit):**
```bash
flow mentions n:abc123
flow mentions .
```

**Behavior (Interactive):**
```bash
flow mentions
# Search for node
```

**Output:**
```
Mentions from: Project: Flow CLI

Outgoing (3):
  → n:x1y2z3  CLI Architecture Design
  → n:a4b5c6  Implementation Tracker
  → n:d7e8f9  Testing Guidelines
```

---

## Tags & Properties

### `flow tag [node-id] [tags...]`

Apply tag(s) to node.

**Arguments:**
- `[node-id]`: Target node (optional)
- `[tags...]`: Tag name(s) (optional, can specify multiple)

**Flags:**
- `--create`: Create tag if it doesn't exist

**Behavior (Explicit):**
```bash
flow tag n:abc123 project
# Applies single tag (promotes to stable if was temp)

flow tag t:a3f821 urgent important
# Applies multiple tags, promotes t:a3f821 to stable ID

flow tag . project active
# Tags current context node
```

**Behavior (Interactive):**
```bash
flow tag
# Search for node → select from existing tags (fuzzy search)
# Can select multiple tags

flow tag abc-123-def
# Shows tag picker with existing tags
# Type to filter, Space to select multiple, Enter to confirm
```

**Tag Picker UI:**
```
Select tags: proj_

  [x] project (47 nodes)
  [ ] project-archived (12 nodes)
  [x] active (23 nodes)
  
  [Create new tag: 'proj'] (press 'n')

[Space Toggle | Enter Confirm | n New | ESC Cancel]
```

**Output:**
```
Tagged node: n:abc123
Tags: [project, active]
```

---

### `flow untag [node-id] [tags...]`

Remove tag(s) from node.

**Arguments:**
- `[node-id]`: Target node (optional)
- `[tags...]`: Tag name(s) (optional, can specify multiple)

**Flags:**
None

**Behavior (Explicit):**
```bash
flow untag n:abc123 project
# Removes single tag

flow untag n:abc123 urgent important
# Removes multiple tags: [urgent, important]
```

**Behavior (Interactive):**
```bash
flow untag
# Search for node → select from node's tags

flow untag abc-123-def
# Shows node's current tags, select to remove
```

**Output:**
```
Removed tags: [urgent, important]
From node: n:abc123
```

---

### `flow prop set [node-id] <key> [value]`

Set property on node.

**Arguments:**
- `[node-id]`: Target node (optional)
- `<key>`: Property key
- `[value]`: Property value (optional, opens editor if omitted)

**Flags:**
- `--type <type>`: Explicit type (string, number, date, reference, list)

**Behavior (Explicit):**
```bash
flow prop set n:abc123 status "in-progress"
flow prop set . priority 1 --type number
flow prop set n:abc123 due @2024-12-01 --type date
```

**Behavior (Interactive):**
```bash
flow prop set
# Search for node → enter key → enter value

flow prop set abc-123-def status
# Enter value only
```

**Output:**
```
Set property on: n:abc123
  status:: in-progress
```

---

### `flow prop get [node-id] <key>`

Get property value.

**Arguments:**
- `[node-id]`: Target node (optional)
- `<key>`: Property key

**Flags:**
None

**Behavior (Explicit):**
```bash
flow prop get n:abc123 status
# Output: in-progress

flow prop get . priority
```

**Behavior (Interactive):**
```bash
flow prop get
# Search for node → select from property list
```

---

### `flow prop delete [node-id] <key>`

Delete property from node.

**Arguments:**
- `[node-id]`: Target node (optional)
- `<key>`: Property key

**Flags:**
- `--force, -f`: Skip confirmation

**Behavior:**
Same pattern as other prop commands.

---

## Queries

### `flow query <sql>`

Execute SQL query and return matching nodes.

**Arguments:**
- `<sql>`: SQL SELECT statement

**Flags:**
- `--format <format>`: Output format (list, tree, ids, table)
- `--limit <n>`: Maximum results (overrides SQL LIMIT)
- `--json`: Output as JSON

**SQL Query Syntax:**

Flow treats nodes as rows in a virtual `nodes` table:

```sql
SELECT * FROM nodes WHERE condition [ORDER BY field] [LIMIT n]
```

**Virtual Schema:**
- `id` - Node UUID
- `content` - Node content (TEXT)
- `created` - Creation timestamp
- `modified` - Last modification timestamp
- `author` - Node creator
- `tags` - Array of tag names
- `parent_id` - Parent node ID
- All node properties available as columns

**Examples:**

```bash
# All nodes with project tag
flow query "SELECT * FROM nodes WHERE 'project' IN tags"

# Active projects with high priority
flow query "SELECT * FROM nodes WHERE 'project' IN tags AND status = 'active' AND priority > 3"

# Content search
flow query "SELECT * FROM nodes WHERE content LIKE '%CRDT%'"

# Recent nodes
flow query "SELECT * FROM nodes WHERE created > '@-7d' ORDER BY created DESC"

# Tasks due soon
flow query "SELECT * FROM nodes WHERE 'task' IN tags AND due_date < '@+3d' AND status != 'done'"

# Complex conditions
flow query "SELECT * FROM nodes WHERE ('urgent' IN tags OR priority = 5) AND status != 'done' ORDER BY priority DESC LIMIT 10"

# Nodes in journal entries
flow query "SELECT * FROM nodes WHERE parent_id IN (SELECT id FROM nodes WHERE content LIKE 'journals/%')"
```

**Output:**
```
3 nodes found:

n:a1b2c3  Project: Flow CLI [project, active]
          priority:: 5, status:: active
             
n:d4e5f6  Project: API Design [project, active]  
          priority:: 4, status:: planning
             
n:g7h8i9  Project: Documentation [project, active]
          priority:: 3, status:: active
```

**JSON Output (--json):**
```json
{
  "query": "SELECT * FROM nodes WHERE 'project' IN tags AND status = 'active'",
  "count": 3,
  "nodes": [
    {
      "id": "n:a1b2c3",
      "content": "Project: Flow CLI",
      "tags": ["project", "active"],
      "properties": {
        "priority": 5,
        "status": "active"
      }
    }
  ]
}
```

---

### `flow find <text>`

Full-text search across all nodes. Convenience wrapper for SQL LIKE query.

**Arguments:**
- `<text>`: Search text

**Flags:**
- `--regex, -r`: Use regex pattern
- `--case-sensitive, -i`: Case-sensitive search
- `--tags <tags>`: Limit to nodes with tags

**Examples:**
```bash
flow find "CRDT implementation"
# Equivalent to: SELECT * FROM nodes WHERE content LIKE '%CRDT implementation%'

flow find "TODO" --tags task
# Equivalent to: SELECT * FROM nodes WHERE content LIKE '%TODO%' AND 'task' IN tags

flow find "^#+ " --regex
# Finds all markdown headers
```

**Output:**
```
5 matches found:

n:a1b2c3  Core Architecture
  ...discuss CRDT implementation details...
  
n:d4e5f6  Sync Protocol
  ...Loro CRDT implementation provides...
```

---

### `flow tagged <tag>`

Find all nodes with specific tag. Convenience wrapper for SQL tag query.

**Arguments:**
- `<tag>`: Tag name

**Flags:**
- `--count`: Show count only
- `--with-props`: Show properties

**Examples:**
```bash
flow tagged project
# Equivalent to: SELECT * FROM nodes WHERE 'project' IN tags

flow tagged task --count
# Returns: 47
```

**Output:**
```
47 nodes tagged: project

n:a1b2c3  Project: Flow CLI
n:d4e5f6  Project: API Design
n:g7h8i9  Project: Documentation
...
```

---

## Validation & Recovery

### `flow check`

Validate space integrity and report issues.

**Flags:**
- `--fix`: Attempt automatic fixes for simple issues
- `--verbose`: Show detailed validation progress

**Output:**
```
✓ 2,847 nodes parsed
✓ 23 tags validated
✓ CRDT state consistent

✗ 3 broken references found:

File: journals/2024-11-30.md:42
  Reference: ((n:xyz789))
  Error: Target node does not exist
  Fix: Remove reference or create target node

File: projects/flow.md:103
  Reference: ((t:abc123))
  Error: Temporary ID referenced (invalid across sessions)
  Fix: Promote target node to stable ID first

File: pages/notes.md:67
  Reference: ((n:def456))
  Error: Target node does not exist
  Fix: Remove reference or create target node
```

**JSON Output:**
```json
{
  "command": "check",
  "valid": false,
  "nodes_parsed": 2847,
  "tags_validated": 23,
  "crdt_consistent": true,
  "broken_references": [
    {
      "file": "journals/2024-11-30.md",
      "line": 42,
      "reference": "n:xyz789",
      "error": "target_not_found"
    }
  ]
}
```

---

### `flow restore [timestamp]`

Restore space to a previous state from CRDT history.

**Arguments:**
- `[timestamp]`: Target restore point (optional, interactive if omitted)

**Flags:**
- `--before <timestamp>`: Restore to state before given time
- `--dry-run`: Show what would be restored without applying
- `--file <path>`: Restore only specific file

**Behavior (Explicit):**
```bash
flow restore --before "2024-11-30 14:23:00"
# Restores all files to state before that time

flow restore --file journals/2024-11-30.md --before "2024-11-30 14:23:00"
# Restores only that file
```

**Behavior (Interactive):**
```bash
flow restore
# Shows recent history, allows selecting restore point
```

**Output:**
```
Restore point: 2024-11-30 14:22:59

Files to restore:
  journals/2024-11-30.md (modified 14:23:41)
  projects/flow.md (modified 14:25:03)

Proceed? [y/N] y

✓ Restored 2 files to pre-edit state
✓ Space validation passed
```

---

### `flow fix [issue-type]`

Interactive repair wizard for broken references and other issues.

**Arguments:**
- `[issue-type]`: Type of issue to fix (optional: references, orphans, duplicates)

**Flags:**
- `--auto`: Apply automatic fixes without prompting
- `--dry-run`: Show what would be fixed

**Behavior (Interactive):**
```bash
flow fix
# Runs flow check, then offers to fix each issue

flow fix references
# Only fix broken references
```

**Output:**
```
Found 3 broken references.

Fix broken reference ((n:xyz789)) in journals/2024-11-30.md:42?
  [1] Remove reference
  [2] Search for similar nodes
  [3] Skip
  [4] Skip all remaining

Choice: 2

Similar nodes found:
  [a] n:xyz780 - "Project notes"
  [b] n:xyz798 - "Meeting summary"
  [c] Create new node with this ID
  [d] Back

Choice: a

✓ Updated reference to n:xyz780
```

---

### `flow promote [node-id]`

Promote a temporary node to stable (permanent) ID.

**Arguments:**
- `[node-id]`: Temporary node ID to promote (optional)

**Flags:**
- `--all`: Promote all temporary nodes in current file
- `--file <path>`: Promote all temporary nodes in specified file

**Behavior (Explicit):**
```bash
flow promote t:a3f821
# Promotes single node

flow promote --file journals/2024-11-30.md
# Promotes all temp nodes in file
```

**Behavior (Interactive):**
```bash
flow promote
# Search for node to promote
```

**Output:**
```
Promoted: t:a3f821 → n:x9m4k1
File: journals/2024-11-30.md
Content: "Important task to remember"
```

**JSON Output:**
```json
{
  "command": "promote",
  "old_id": "t:a3f821",
  "new_id": "n:x9m4k1",
  "file": "journals/2024-11-30.md",
  "content": "Important task to remember"
}
```

---

### `flow history --external`

Show changes made outside of Flow (external editor edits).

**Flags:**
- `--since <timestamp>`: Show changes since given time
- `--file <path>`: Filter to specific file

**Output:**
```
External changes detected:

2024-11-30 14:23:41 - journals/2024-11-30.md
  Source: vim
  Lines changed: 12

2024-11-30 14:25:03 - projects/flow.md
  Source: vscode
  Lines changed: 5

Use 'flow restore --before <timestamp>' to revert changes.
```

---

## Batch Operations

### `flow import <file>`

Import markdown file as nodes.

**Arguments:**
- `<file>`: Markdown file path

**Flags:**
- `--parent <node-id>`: Import under specific parent
- `--preserve-structure`: Maintain heading hierarchy
- `--tags <tags>`: Tag all imported nodes (comma-separated)

**Examples:**
```bash
flow import notes.md
flow import project.md --parent n:abc123 --tags imported,archive
```

**Output:**
```
Imported 15 nodes from notes.md
Root node: n:x9y8z7
Tags applied: [imported, archive]
```

---

### `flow import logseq <path>`

Import a Logseq graph into Flow.

**Arguments:**
- `<path>`: Path to Logseq graph directory

**Flags:**
- `--dry-run`: Show what would be imported without making changes
- `--preserve-properties`: Keep Logseq-specific properties (collapsed, etc.)

**Behavior:**
```bash
flow import logseq ~/logseq-notes
# Converts Logseq graph to Flow space
```

**Conversion process:**
1. Scans all markdown files
2. Converts UUID references to Flow IDs
3. Strips Logseq-specific properties (unless --preserve-properties)
4. Rewrites all `((uuid))` references
5. Inserts HTML comments for stable IDs
6. Validates converted space

**Output:**
```
Importing Logseq graph from: ~/logseq-notes

Phase 1: Scanning...
  Found 847 files
  Found 12,453 nodes
  Found 3,891 references

Phase 2: Converting...
  ✓ IDs converted: 12,453
  ✓ References rewritten: 3,891
  ✓ Properties cleaned: 2,104

Phase 3: Validating...
  ✓ All references valid
  ✓ No broken links

Import complete!
  Files: 847
  Nodes: 12,453
  References: 3,891
```

---

### `flow export [node-id] [file]`

Export node subtree to markdown.

**Arguments:**
- `[node-id]`: Root node to export (optional)
- `[file]`: Output file path (optional, defaults to stdout)

**Flags:**
- `--depth <n>`: Limit export depth
- `--with-properties`: Include property frontmatter
- `--with-ids`: Include node IDs as comments

**Behavior (Explicit):**
```bash
flow export n:abc123 output.md
flow export @today today.md --with-properties
```

**Behavior (Interactive):**
```bash
flow export
# Search for node → enter filename
```

---

### `flow batch <script>`

Execute batch operations from file.

**Arguments:**
- `<script>`: Script file with commands (one per line)

**Flags:**
- `--dry-run`: Show what would be executed
- `--continue-on-error`: Don't stop on first error

**Script Format:**
```bash
# comments allowed
create "New project" --tag project
tag $LAST_ID active
prop set $LAST_ID status "planning"
```

**Special Variables:**
```
$LAST_ID                              # ID from previous command
$TODAY                                # Expands to @today
```

---

## Server Integration (Phase 4)

### `flow sync push`

Push local changes to sync server.

**Flags:**
- `--force, -f`: Force push (overwrite conflicts)
- `--dry-run`: Show what would be pushed

---

### `flow sync pull`

Pull changes from sync server.

**Flags:**
- `--force, -f`: Force pull (overwrite local changes)
- `--dry-run`: Show what would be pulled

---

### `flow sync status`

Show sync state and pending changes.

**Output:**
```
Sync Status: Connected
Server: https://sync.example.com
Last sync: 2024-11-24 10:15:00

Pending push: 5 changes
Pending pull: 2 changes
Conflicts: 0
```

---

### `flow serve`

Start local sync server.

**Flags:**
- `--port <port>`: Server port (default: 8080)
- `--host <host>`: Bind address (default: 127.0.0.1)
- `--public`: Allow external connections

---

## Metadata Commands

### `flow info [node-id]`

Show detailed node metadata.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
None

**Output:**
```
Node: n:abc123
Type: Stable
Created: 2024-11-20 14:22:00
Modified: 2024-11-24 10:15:00
File: pages/projects/flow.md

Tags: [project, active]
Properties: 2
Children: 3
References: 8 (3 outgoing, 5 incoming)
```

---

### `flow history [node-id]`

Show node edit history.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--limit <n>`: Limit history entries
- `--diff`: Show content diffs

**Output:**
```
History for: abc-123-def

2024-11-24 10:15:00  michael
  Modified content
  
2024-11-23 16:30:00  michael
  Added property: status=in-progress
  
2024-11-20 14:22:00  michael
  Created node
```

---

### `flow related [node-id]`

Suggest related nodes based on content/references.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--limit <n>`: Maximum suggestions
- `--algorithm <algo>`: Similarity algorithm (content, space, combined)

---

## Configuration

### Space Registry

All initialized spaces are registered in `~/.config/flow/spaces.toml`:

```toml
[spaces.personal]
path = "/home/user/notes"
created = "2024-11-20T14:22:00Z"

[spaces.work]
path = "/home/user/work/flow"
created = "2024-11-24T10:15:00Z"
```

This enables name-based space access via `flow open <name>`.

### `flow config get <key>`

Get configuration value.

**Examples:**
```bash
flow config get default_space
flow config get editor
```

---

### `flow config set <key> <value>`

Set configuration value.

**Examples:**
```bash
flow config set default_space /path/to/space
flow config set editor nvim
```

---

### `flow config unregister <name>`

Remove space from registry (does not delete files).

**Arguments:**
- `<name>`: Registered space name

**Examples:**
```bash
flow config unregister old-project
```

**Output:**
```
Unregistered space: old-project
Path: /path/to/space (files preserved)
```

---

### `flow config rename <old-name> <new-name>`

Rename registered space.

**Arguments:**
- `<old-name>`: Current space name
- `<new-name>`: New space name

**Examples:**
```bash
flow config rename work work-archive
```

**Output:**
```
Renamed space: work → work-archive
```

---

## Error Handling

All commands use consistent exit codes:

- `0`: Success
- `1`: General error (invalid input, not found, etc.)
- `2`: Permission/access error
- `3`: Space corruption/integrity error
- `130`: User cancelled (Ctrl-C, ESC in interactive mode)

Error messages written to stderr, results to stdout.

## Output Formats

### Default (Human-Readable)
Formatted for terminal display with colors and formatting.

### JSON (`--json`)
Machine-parsable JSON for scripting:

**JSON Output (--json):**
```json
{
  "command": "show",
  "node": {
    "id": "n:abc123",
    "content": "Node content",
    "tags": ["tag1"],
    "properties": {},
    "created": "2024-11-20T14:22:00Z",
    "modified": "2024-11-24T10:15:00Z"
  }
}
```

### Raw (`--raw` where applicable)
Unformatted content only, suitable for piping.

## Interactive Search Behavior

All interactive searches support:

- Fuzzy matching on content and metadata
- Real-time filtering as you type
- Keyboard navigation (arrow keys or vim-style hjkl)
- Context preview (parent/children shown)
- Multiple selection modes where applicable
- ESC to cancel, Enter to confirm
- Tab for autocomplete on partial matches

Search results ranked by:
1. Exact matches
2. Recent modifications
3. Content relevance
4. Reference frequency

## Environment Variables

```
FLOW_SPACE          Default space path
FLOW_EDITOR         Editor command (overrides $EDITOR)
FLOW_NO_COLOR       Disable color output
FLOW_CONFIG_HOME    Config directory (default: ~/.config/flow)
```

## Shell Completion

Commands support shell completion for:
- bash
- zsh
- fish

Generate with: `flow completion <shell>`
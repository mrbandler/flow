# Flow CLI Command Specification

## Design Principles

1. **Dual Mode Operation**: Commands without required node IDs enter interactive mode
2. **Context Persistence**: Last accessed node stored as `.` reference
3. **Date References**: Journal nodes addressable by date expressions
4. **Inline Syntax**: Tags (#tag) and properties (key::value) parsed from content strings
5. **Machine Parsable**: JSON output via `--json` flag
6. **Unix Philosophy**: Commands chainable via pipes, scriptable

## Global Flags

Available for all commands:

```
--json              Output in JSON format
--graph <path>      Target specific graph (overrides active)
--verbose, -v       Detailed logging
--quiet, -q         Suppress non-error output
--help, -h          Command help
```

## Context System

Context (`.`) refers to the last accessed node in the current session. Persisted in graph's `.flow/context` file.

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
flow create "Project Alpha #project #active #q4"
# Content: "Project Alpha"
# Tags: [project, active, q4]

flow add "Meeting with team #meeting #important"
# Content: "Meeting with team"
# Tags: [meeting, important]
```

### Property Syntax

Properties use `key::value` format:

```bash
flow create "Bug fix status::in-progress priority::high"
# Content: "Bug fix"
# Properties: {status: "in-progress", priority: "high"}

flow create "Research date::2024-11-25 owner::michael"
# Content: "Research"
# Properties: {date: "2024-11-25", owner: "michael"}
```

### Multi-word Values

Quote values containing spaces:

```bash
flow create "Task status::\"needs review\" assignee::\"Alice Smith\""
# Properties: {status: "needs review", assignee: "Alice Smith"}
```

### Combined Syntax

Tags and properties can be mixed:

```bash
flow create "Design Review #meeting date::2024-11-25 attendees::5 #important"
# Content: "Design Review"
# Tags: [meeting, important]
# Properties: {date: "2024-11-25", attendees: "5"}
```

### Property Type Detection

Property values are automatically typed:

```
priority::5              → number
due::2024-11-25         → date
status::done            → string
tags::[ui,ux,design]    → list
owner::@abc-123-def     → reference (@ prefix for node IDs)
completed::true         → boolean
```

### Parsing Rules

1. Extract all `#tag` patterns and apply as tags
2. Extract all `key::value` patterns and apply as properties
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

Suggests existing tags in graph, frequency-ranked.

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
- Existing tags in graph
- Property keys used in graph  
- Property values for each key (context-aware)
- Frequency-ranked (most used suggestions first)

### Explicit Flags vs Inline Syntax

Inline syntax is the primary method. Explicit flags remain available for scripting:

```bash
# Inline (preferred for interactive use)
flow create "Task #urgent priority::high"

# Explicit flags (preferred for scripting)
flow create "Task" --tags urgent --property priority=high

# Mixed usage (flags override inline)
flow create "Task #work" --tags urgent,important
# Results in tags: [urgent, important] (inline #work ignored)
```

---

## Graph Management Commands

### `flow init <path>`

Initialize a new Flow graph.

**Arguments:**
- `<path>`: Directory path for new graph

**Flags:**
- `--name <name>`: Graph name (defaults to directory name)
- `--template <template>`: Initialize with template structure

**Behavior:**
- Creates directory structure
- Initializes Loro container
- Creates journal directory
- Writes graph metadata

**Output:**
```
Initialized Flow graph at /path/to/graph
Graph ID: abc-123-def
```

**Exit Codes:**
- 0: Success
- 1: Path already exists
- 2: Permission denied

---

### `flow open <path|name>`

Set the active graph for subsequent commands.

**Arguments:**
- `<path|name>`: Path to existing graph or registered graph name

**Flags:**
- `--set-default`: Make this the default graph

**Behavior:**
- Accepts either full/relative path or graph name
- If name provided, looks up in `~/.config/flow/graphs.toml`
- If path provided, opens directly
- Validates graph directory
- Updates `~/.config/flow/active_graph`
- Loads graph metadata

**Examples:**
```bash
flow open /home/user/notes          # Absolute path
flow open ~/work/flow-graph         # Relative path
flow open personal                  # Registered name
flow open work                      # Registered name
```

**Output:**
```
Opened graph: GraphName
Path: /path/to/graph
```

**Exit Codes:**
- 0: Success
- 1: Graph not found (name lookup failed or invalid path)
- 2: Graph corrupted

---

### `flow list`

List all registered graphs.

**Flags:**
- `--verbose, -v`: Show graph statistics

**Output (default):**
```
* personal (/home/user/notes) [active]
  work (/home/user/work/flow)
  archive (/mnt/archive/old-notes)
```

**Output (--json):**
```json
{
  "graphs": [
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

### `flow status`

Show current graph health and statistics.

**Flags:**
- `--check-integrity`: Validate CRDT state
- `--show-conflicts`: List unresolved conflicts

**Output:**
```
Graph: GraphName
Path: /path/to/graph
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
flow add "Meeting notes #meeting status::done"
# Creates node with content "Meeting notes", tag [meeting], property {status: "done"}

flow add "Task #urgent #important priority::high"
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
Created node: abc-123-def
In: journal/2024-11-24.md
Tags: [meeting]
Properties: {status: "done"}
```

---

### `flow create [content]`

Create standalone node (not in journal hierarchy).

**Arguments:**
- `[content]`: Node content with optional inline tags/properties (optional)

**Flags:**
- `--parent <node-id>`: Create as child of parent
- `--tags <tags>`: Explicit tags (comma-separated, overrides inline)
- `--property <key>=<value>`: Explicit property (repeatable, overrides inline)

**Behavior (Explicit):**
```bash
flow create "Project: Flow CLI #project #active status::planning"
# Content: "Project: Flow CLI"
# Tags: [project, active]
# Properties: {status: "planning"}
# Returns: xyz-789-abc

flow create "Task #urgent priority::high owner::michael"
# Tags: [urgent], properties: {priority: "high", owner: "michael"}

# Explicit flags (scripting)
flow create "Flow CLI" --tags project,active --property status=planning
```

**Behavior (Interactive):**
```bash
flow create
# Opens $EDITOR with autocomplete, creates on save
```

**Output:**
```
Created node: xyz-789-abc
Tags: [project, active]
Properties: {status: "planning"}
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
flow edit abc-123-def
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
Editing node: abc-123-def
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
flow show abc-123-def
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
Node: abc-123-def
Created: 2024-11-20 14:22:00
Modified: 2024-11-24 10:15:00
Tags: [project, active]

# Project: Flow CLI

Implementing the CLI interface for Flow.

Properties:
  status: in-progress
  priority: high

References:
  → xyz-111-222 (CLI Architecture Design)
  ← def-333-444 (Implementation Tracker)

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
flow delete abc-123-def
# Prompts: "Delete node 'Project: Flow CLI'? [y/N]"

flow delete abc-123-def --recursive --force
# Deletes without confirmation
```

**Behavior (Interactive):**
```bash
flow delete
# Search → select → confirm deletion
```

**Output:**
```
Deleted node: abc-123-def
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
flow move abc-123-def xyz-789-abc
# Moves abc-123-def under xyz-789-abc

flow move abc-123-def @today
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
Moved node: abc-123-def
New parent: xyz-789-abc
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
flow append abc-123-def "Subtask #task status::todo"
# Creates child under abc-123-def with tag [task] and property {status: "todo"}

flow append @today "Quick note #reminder"
# Adds to today's journal with tag [reminder]

# Explicit flags (scripting)
flow append abc-123-def "Subtask" --tags task,urgent --property status=todo
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
flow tree abc-123-def
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
│   ├── Graph module
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
    "id": "abc-123-def",
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
flow children abc-123-def
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

  abc-111-111  Core Architecture
  abc-222-222  CLI Implementation [in-progress]
  abc-333-333  Testing Strategy

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
flow parent abc-123-def
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

xyz-789-abc  Project: Flow CLI
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
flow path abc-222-222
```

**Output:**
```
journal/2024-11-20.md
  → Project: Flow CLI
    → CLI Implementation
```

---

## References

### `flow link [source] [target]`

Create reference between nodes.

**Arguments:**
- `[source]`: Source node (optional)
- `[target]`: Target node (optional)

**Flags:**
- `--bidirectional, -b`: Create mutual references

**Behavior (Explicit):**
```bash
flow link abc-123-def xyz-789-abc
# Creates reference from abc to xyz

flow link . xyz-789-abc
# Link from current context
```

**Behavior (Interactive):**
```bash
flow link
# Search for source → search for target

flow link abc-123-def
# Search for target only
```

**Output:**
```
Created reference: abc-123-def → xyz-789-abc
```

---

### `flow unlink [source] [target]`

Remove reference between nodes.

**Arguments:**
- `[source]`: Source node (optional)
- `[target]`: Target node (optional)

**Flags:**
- `--force, -f`: Skip confirmation

**Behavior:**
Same pattern as `link` command.

**Output:**
```
Removed reference: abc-123-def → xyz-789-abc
```

---

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
flow refs abc-123-def
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
  → xyz-111-222  CLI Architecture Design
  → xyz-222-333  Implementation Tracker
  → xyz-333-444  Testing Guidelines

Incoming (5):
  ← def-111-111  Q4 Projects
  ← def-222-222  Active Development
  ← def-333-333  Code Review Notes
  ← def-444-444  Sprint Planning
  ← def-555-555  Team Updates
```

---

### `flow backlinks [node-id]`

Show incoming references only.

**Arguments:**
- `[node-id]`: Target node (optional)

**Flags:**
- `--context, -c`: Show surrounding content

**Behavior:**
Same pattern as `refs` but filtered to incoming only.

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
flow tag abc-123-def project
# Applies single tag

flow tag abc-123-def urgent important
# Applies multiple tags: [urgent, important]

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
Tagged node: abc-123-def
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
flow untag abc-123-def project
# Removes single tag

flow untag abc-123-def urgent important
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
From node: abc-123-def
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
flow prop set abc-123-def status "in-progress"
flow prop set . priority 1 --type number
flow prop set abc-123-def due @2024-12-01 --type date
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
Set property on: abc-123-def
  status: in-progress
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
flow prop get abc-123-def status
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
flow query "SELECT * FROM nodes WHERE parent_id IN (SELECT id FROM nodes WHERE content LIKE 'journal/%')"
```

**Output:**
```
3 nodes found:

abc-111-111  Project: Flow CLI [project, active]
             priority: 5, status: active
             
abc-222-222  Project: API Design [project, active]  
             priority: 4, status: planning
             
abc-333-333  Project: Documentation [project, active]
             priority: 3, status: active
```

**JSON Output (--json):**
```json
{
  "query": "SELECT * FROM nodes WHERE 'project' IN tags AND status = 'active'",
  "count": 3,
  "nodes": [
    {
      "id": "abc-111-111",
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

abc-111-111  Core Architecture
  ...discuss CRDT implementation details...
  
abc-222-222  Sync Protocol
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

abc-111-111  Project: Flow CLI
abc-222-222  Project: API Design
abc-333-333  Project: Documentation
...
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
flow import project.md --parent abc-123-def --tags imported,archive
```

**Output:**
```
Imported 15 nodes from notes.md
Root node: xyz-999-999
Tags applied: [imported, archive]
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
flow export abc-123-def output.md
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
Node: abc-123-def
UUID: 123e4567-e89b-12d3-a456-426614174000
Created: 2024-11-20 14:22:00
Modified: 2024-11-24 10:15:00
Author: michael
File: nodes/abc-123-def.md

Tags: [project, active]
Properties: 2
Children: 3
References: 8 (3 outgoing, 5 incoming)

CRDT Version: 47
Conflicts: None
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
- `--algorithm <algo>`: Similarity algorithm (content, graph, combined)

---

## Configuration

### Graph Registry

All initialized graphs are registered in `~/.config/flow/graphs.toml`:

```toml
[graphs.personal]
path = "/home/user/notes"
created = "2024-11-20T14:22:00Z"

[graphs.work]
path = "/home/user/work/flow"
created = "2024-11-24T10:15:00Z"
```

This enables name-based graph access via `flow open <name>`.

### `flow config get <key>`

Get configuration value.

**Examples:**
```bash
flow config get default_graph
flow config get editor
```

---

### `flow config set <key> <value>`

Set configuration value.

**Examples:**
```bash
flow config set default_graph /path/to/graph
flow config set editor nvim
```

---

### `flow config unregister <name>`

Remove graph from registry (does not delete files).

**Arguments:**
- `<name>`: Registered graph name

**Examples:**
```bash
flow config unregister old-project
```

**Output:**
```
Unregistered graph: old-project
Path: /path/to/graph (files preserved)
```

---

### `flow config rename <old-name> <new-name>`

Rename registered graph.

**Arguments:**
- `<old-name>`: Current graph name
- `<new-name>`: New graph name

**Examples:**
```bash
flow config rename work work-archive
```

**Output:**
```
Renamed graph: work → work-archive
```

---

## Error Handling

All commands use consistent exit codes:

- `0`: Success
- `1`: General error (invalid input, not found, etc.)
- `2`: Permission/access error
- `3`: Graph corruption/integrity error
- `130`: User cancelled (Ctrl-C, ESC in interactive mode)

Error messages written to stderr, results to stdout.

## Output Formats

### Default (Human-Readable)
Formatted for terminal display with colors and formatting.

### JSON (`--json`)
Machine-parsable JSON for scripting:

```json
{
  "command": "show",
  "node": {
    "id": "abc-123-def",
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
FLOW_GRAPH          Default graph path
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
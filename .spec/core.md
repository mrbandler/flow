# Flow Core Architecture Specification

## Overview

Flow is a privacy-focused, local-first note-taking system where **markdown files are the absolute source of truth**. The system uses a multi-layer architecture with CRDT sync capabilities and full-text search.

**Core Principle:** All data originates from markdown files → everything else is derived.

**Design Philosophy:** Flow is a tree-based outliner with cross-references, not a graph database. The architecture uses efficient lookups and explicit indices instead of graph algorithms. The "space" terminology represents an isolated workspace for the user.

## Terminology

**User-facing:** "Space" - A Flow Space is your workspace (e.g., "Flow Space", "personal space", "work space")
**Internal:** "Graph" - The in-memory data structure representing all nodes and their relationships

## Architecture Layers

```
Markdown Files (Source of Truth)
  ↓ import/export
CRDT Storage (Persistence & Sync)
  ↓ parse
FlowNode AST (Custom Syntax)
  ↓ convert
In-Memory Graph
  ↓ index
Full-Text Search Engine
```

**Layer responsibilities:**

1. **Markdown Files** - Human-editable, external editor compatible, git-friendly
2. **CRDT Storage** - Stores entire markdown file as CRDT text, handles multi-device sync
3. **FlowNode AST** - Custom AST where Flow syntax (properties, tags, references) are first-class nodes
4. **Graph** - Efficient structure for fast node access and traversal
5. **Search Engine** - Full-text search for content queries and unlinked mentions

## Data Model

### Node Identity

Flow uses a hybrid ID system with two types of node identities:

**Temporary IDs (position-based):**
- Format: `t:abc123` (6-character hash)
- Generated from file path, row, and column position
- Regenerated on each parse based on position
- Used for unpromoted nodes
- Valid only within current session
- No visible marker in markdown
- Enables stable CLI JSON output for workflow chains

**Stable IDs (permanent):**
- Format: `n:abc123` (6-character unique identifier)
- Embedded in markdown as HTML comment: `<!-- n:abc123 -->`
- Survives external edits and re-imports
- Created when node is promoted (first reference/tag/property)
- Persistent across sessions and devices
- Required for object model features

**ID Prefix Convention:**
- `t:` prefix = temporary (position hash)
- `n:` prefix = stable (permanent identifier)

**Promotion Triggers:**

A temporary node gets promoted to stable when:
1. **First reference** - Another node references it via `((node-id))`
2. **First tag** - Node receives a tag (`#tagname`)
3. **First property** - Node receives a property (`key:: value`)
4. **Explicit promotion** - User explicitly promotes a node

**Promotion Process:**
1. Generate unique 6-character identifier
2. Create stable ID with `n:` prefix
3. Embed HTML comment in node content: `<!-- n:abc123 -->`
4. Update node ID in Graph (remove temp entry, re-insert as stable)
5. Update all indices (by_file, by_tag, backlinks)
6. Render file to markdown (now includes HTML comment)
7. Update CRDT storage with new file content

**Hash Collision Handling:**

Position-based hashes have millions of combinations. Collisions within a single session are negligible. If collision occurs:
- Detected during Graph insertion (ID already exists)
- Append position suffix to disambiguate
- Rare edge case, acceptable for temporary IDs

### Markdown Syntax

**Standard outliner:**
```markdown
- Node content
  - Child node
    - Grandchild
  - Another child
```

**Flow extensions with stable ID:**
```markdown
- Task node <!-- n:abc123 -->
  status:: in-progress
  priority:: 5
  due:: 2024-12-01
  #project #urgent
  - Subtask one <!-- n:def456 -->
  - Subtask two
```

Note: Properties use `key:: value` syntax (space after `::`).

**Reference types:**
- `((abc123))` - Bare reference (promotes target if temp)
- `[Custom text](((abc123)))` - Aliased reference
- `{{embed ((abc123))}}` - Transclude content
- `((abc123))^` - Creation marker (node was created here, now lives elsewhere)

**Syntax rules:**
- HTML comment `<!-- n:... -->` marks stable ID (only present after promotion)
- `key:: value` - Properties (arbitrary key-value pairs)
- `#tagname` - Tags (inline in content)
- Properties must be on indented lines below node content
- Tags can appear anywhere in content text
- HTML comments should appear at end of node's first line

### Graph Structure

**Primary storage:**
- All nodes keyed by ID for O(1) access

**Fast lookup indices:**
- **by_file** - All nodes in a file
- **by_tag** - All nodes with a specific tag
- **backlinks** - Which nodes reference each node

**Node structure:**
- `id` - Either temporary or stable with prefix
- `file_path` - Location in filesystem
- `position` - Row and column for temp ID generation
- `text` - Clean content (properties/tags/comments removed)
- `parent` - Parent node reference
- `children` - Child node references
- `tags` - Extracted tags
- `properties` - Typed key-value pairs
- `created_at, modified_at` - Timestamps

**Property types:**
- Text, Number, Boolean, Date (ISO 8601), Reference (node-id), List

**Why efficient lookup over graph library:**
- O(1) node lookup by ID
- No need for graph algorithms (shortest path, cycle detection, etc.)
- Outliner is a tree (acyclic by design)
- Simple traversal sufficient (parent → children, child → parent)
- Custom indices give us the queries we need
- Easier to understand, debug, and serialize

**Traversal operations:**
- Get children: Access node's children list, lookup each in storage
- Get parent: Access node's parent, lookup in storage
- Get ancestors: Walk parent chain until none
- Get descendants: Recursively collect children
- Get siblings: Get parent's children excluding self

## Markdown Parsing

### Parser Selection

**Requirements:**
- Fast parsing performance
- Event-based streaming (memory efficient)
- Well-tested and maintained
- Simple to post-process for custom syntax

**Parsing pipeline:**

```
Markdown text
  ↓ 
Parser events
  ↓
FlowNode AST
  ↓
Graph
```

### Stage 1: Event Processing

The parser emits events as it parses:
- List boundaries (start/end)
- List item boundaries (start/end)
- Text content
- Inline code
- Other markdown elements

Track list depth via nested list tags. Accumulate text between item start/end. Each item becomes a potential node. Track position (row, col) for each node during parsing.

### Stage 2: FlowNode AST Construction

**FlowNode types:**

- **Root** - Top level container with children
- **ListItem** - Individual node with depth, ID marker, position, text, properties, tags, references, and children

**Custom syntax extraction:**

For each list item's accumulated text:

1. **HTML Comment ID** - Extract `<!-- n:abc123 -->` pattern
   - If found: Mark as stable ID, remove comment from text
   - If not found: Will generate temp ID from position later

2. **Properties** - Lines matching `key:: value` pattern
   - Extract key and value
   - Remove from text content
   - Parse value type (number, boolean, date, reference, or text)

3. **Tags** - Patterns matching `#word` in text
   - Extract all tag names
   - Keep tags in text (they're content, not metadata)

4. **References** - Multiple patterns to detect:
   - `{{embed ((id))}}` - Must check first (most specific)
   - `[text](((id)))` - Aliased reference
   - `((id))^` - Creation marker (must check before bare)
   - `((id))` - Bare reference
   - Each creates appropriate reference variant

**Reference variants:**
- Bare - Simple node reference
- Aliased - Reference with custom display text
- Embed - Transclusion
- CreationMarker - Indicates where node was originally created

### Stage 3: AST to Graph Conversion

Build Graph from FlowNode tree:

1. **Process recursively** - Depth-first traversal of FlowNode tree
2. **Assign IDs**:
   - If HTML comment found: Use stable ID `n:abc123`
   - Else: Generate temp ID from position hash
3. **Create Node** - With all fields populated from FlowNode
4. **Build relationships** - Set parent reference, populate children list
5. **Build indices** - Add to by_file, by_tag, backlinks maps
6. **Process children** - Recurse for all child FlowNodes

**Result:** Flat collection of nodes with ID-based relationships and indices.

## CRDT Integration

### File Storage Model

**Structure:**
- Map of file paths to file containers
- Each container has:
  - **content** - Entire markdown file as CRDT text
  - **metadata** - File path, timestamps, etc.

**Metadata tracked:**
- `path` - File path relative to space root
- `modified_at` - When CRDT was last updated
- `mtime` - Filesystem mtime for change detection

**Why CRDT text per file:**
- CRDT text operations handle concurrent edits automatically
- Character-level conflict resolution
- Simple sync protocol (text deltas)
- No complex reconciliation needed
- Entire file is atomic unit for CRDT

### Sync Protocol

**Local edit flow:**
1. User edits node in Flow
2. Update in-memory Graph
3. Render entire file to markdown (includes HTML comments for stable IDs)
4. Update CRDT: Replace entire file content
5. CRDT generates operation
6. Operation synced to server
7. Server broadcasts to other devices

**Remote update flow:**
1. Receive CRDT operations from server
2. CRDT applies operations (automatic merge)
3. Subscriber notified of file content change
4. Re-parse markdown from CRDT
5. Rebuild Graph for affected file (regenerate temp IDs, extract stable IDs)
6. Update search index
7. Export to disk (debounced)
8. UI updates

**Conflict resolution:**
CRDT text automatically merges concurrent character-level edits. No manual intervention needed. Both devices converge to same text state.

**Example concurrent edits:**
- Device A: "Task" → "Task A"
- Device B: "Task" → "Task B"
- CRDT merge: "Task AB" or "Task BA" (deterministic)
- Both devices converge automatically

**Temp ID regeneration across devices:**
- Temp IDs are position-based, regenerated on each parse
- Same file position = same temp ID hash across devices
- Only matters for stable IDs (which sync via HTML comments)
- Temp IDs never appear in sync protocol

### Change Detection

**On startup:**
- Scan all markdown files in filesystem
- For each file, compare filesystem mtime with CRDT metadata mtime
- If filesystem newer or file not in CRDT: import
- If file in CRDT but not on filesystem: delete from CRDT

**During runtime:**
- File watcher detects changes to markdown files
- Check if export lock exists (ignore if Flow wrote the file)
- Read new content, update CRDT
- CRDT operation logged for sync
- Rebuild Graph for file (regenerate temp IDs, extract stable IDs)

## Full-Text Search

### Search Engine Integration

**Index location:** `.flow/search_index/`

**Schema fields:**
- `id` (stored) - Node ID (stable only, temp IDs not indexed)
- `file_path` (stored) - File location
- `content` (searchable) - Node text content
- `tags` (searchable) - Space-separated tags
- `created_at` (indexed) - Creation timestamp
- `modified_at` (indexed) - Last modified

**Indexing strategy:**

When nodes change:
1. Build search document from node
2. Only index stable nodes (promoted with tags/properties/references)
3. Temp nodes not indexed (transient, no object features)
4. Add to index (or update if exists)
5. Batch commits (not after every change)
6. Periodic commit (e.g., every 100 changes or 1 second)

**Query capabilities:**
- Full-text: `content:rust`
- Tag filtering: `tag:project`
- Boolean: `rust AND project`
- Phrase: `"exact phrase"`
- Field-specific: `content:test tag:urgent`
- Fuzzy search: `rust~2`

### Unlinked Mentions

Unlinked mentions are nodes containing similar text but no explicit reference. This enables discovery of related content.

**Algorithm:**

For a node with text "Testing implementation details":

1. **Extract key terms** - Split text, filter out:
   - Short words (< 4 chars)
   - Common stop words (the, a, and, etc.)
   - Special characters
   - Result: ["Testing", "implementation", "details"]

2. **Build query** - Combine terms with OR

3. **Search index** - Execute query, get top results (stable IDs only)

4. **Filter results** - Exclude:
   - The node itself
   - Nodes that explicitly reference this node (in backlinks)
   - Nodes that this node explicitly references

5. **Return** - Remaining nodes are unlinked mentions

**Use case:**
- Node A: "Testing implementation"
- Node B: "Need to write more tests" ← mention of "test"
- Node C: "See ((node-a-id))" ← explicit reference (excluded)
- Unlinked mentions for A: [Node B]

## Query System

### SQL-like Query Language

Flow uses SQL-like syntax for queries, targeting developers who already know SQL:

```sql
-- Simple tag filter
SELECT * FROM nodes WHERE 'project' IN tags;

-- Property filters
SELECT * FROM nodes 
WHERE 'task' IN tags 
AND properties.priority > 5;

-- Text search + filters
SELECT * FROM nodes 
WHERE content CONTAINS 'rust'
AND 'urgent' IN tags;

-- Backlinks
SELECT * FROM nodes 
WHERE id IN (SELECT source FROM backlinks WHERE target = 'n:abc123');
```

**Query execution:**

1. Start with all nodes
2. Apply each filter in sequence
3. Sort results if specified
4. Apply limit if specified
5. Return matching nodes

**Future: User-defined queries** - Users can save queries as "search nodes" that dynamically update.

**Rationale:** Developers know SQL. More esoteric query languages add unnecessary learning curve. SQL-like syntax is familiar and sufficient for our needs.

## External Editor Support & Validation

### Broken Reference Detection

Flow accepts that users may edit markdown in external editors and break references. The system detects and reports these issues clearly.

**Broken reference information:**
- Source node ID
- Target ID that doesn't exist
- Reference type (bare, aliased, embed)
- File path
- Position in file

**Validation process:**

For each node, check all references. If a reference target doesn't exist in the graph, record it as broken.

**CLI validation example:**
```bash
$ flow check
✓ 2,847 nodes parsed
✗ 3 broken references found:

File: journals/2024-11-30.md:42
  Reference: ((n:xyz789))
  Error: Target node does not exist
  Fix: Remove reference or create target node

File: projects/flow.md:103
  Reference: ((t:abc123))
  Error: Temporary ID referenced (invalid across sessions)
  Fix: Promote target node to stable ID first
```

### CRDT-Based Recovery

When external edits corrupt the space, CRDT history provides recovery:

**Capabilities:**
- Show what changed outside Flow since a given timestamp
- Filter to only external (non-Flow) changes
- Restore to a specific point in time

**Recovery workflow:**
```bash
$ flow check
✗ 12 broken references after external edit

$ flow history --external
2024-11-30 14:23:41 - projects/flow.md (vim)
2024-11-30 14:25:03 - journals/today.md (vim)

$ flow restore --before "2024-11-30 14:23:00"
✓ Restored 2 files to pre-edit state
✓ Space validation passed
```

**Design philosophy:** We don't prevent corruption from external edits. Instead, we detect problems clearly and provide easy recovery via CRDT version history.

## Logseq Import

### Import Requirements

Flow provides one-way import from Logseq to Flow. This is a conversion tool, not bidirectional sync.

**ID Conversion:**
- Logseq uses UUID format: `id:: 60a78b6b-b74f-4496-a7b7-dc0d454ca4f3`
- Flow uses short ID format: `<!-- n:a3k9m2 -->`
- Build UUID→Flow ID mapping, rewrite all references

**Property Handling:**
- Keep user properties: `status:: done`, `priority:: 5`
- Strip Logseq-specific: `collapsed:: true`, `card-*`, `id::`
- Convert heading levels: `heading:: 2` → markdown `##`

**Reference Rewriting:**
- `((uuid))` → `((n:flowid))`
- `{{embed ((uuid))}}` → `{{embed ((n:flowid))}}`
- `[Label](((uuid)))` → `[Label](((n:flowid)))`

**Import Process:**

1. **Phase 1: Scan** - Scan all files, build UUID→Flow ID map
2. **Phase 2: Convert** - For each file:
   - Read content
   - Remove `id::` properties
   - Rewrite all UUID references using map
   - Insert HTML comments for stable IDs
   - Clean Logseq-specific properties
   - Write converted content
3. **Phase 3: Initialize** - Initialize Flow Space
4. **Phase 4: Validate** - Load graph and check for broken references

**Import validation:**
- All UUID references mapped to Flow IDs
- No broken references after conversion
- Properties preserved (minus Logseq-specific ones)
- File structure maintained
- Space builds successfully

**Note:** All imported nodes are immediately promoted to stable IDs, since they were already stable in Logseq.

## Complete Operation Flows

### Startup

**Goal:** Initialize Space from markdown files on disk

**Steps:**

1. **Discover files** - Walk journals/ and pages/ recursively, collect .md files
2. **Load CRDT** - Load .flow/space.loro or create new document
3. **Import changed files** - For each markdown file:
   - Compare filesystem mtime with CRDT metadata mtime
   - If newer or missing: Read file, update CRDT content, update metadata
4. **Detect deletions** - Files in CRDT but not on filesystem: delete from CRDT
5. **Parse all files (parallel)** - For each file in CRDT:
   - Get markdown text from CRDT
   - Parse markdown → events (track positions)
   - Build FlowNode AST → extract HTML comments for stable IDs
   - Convert to Graph nodes → generate temp IDs from position hashes
6. **Build indices** - Populate by_file, by_tag, backlinks
7. **Build search index** - Index all stable nodes
8. **Validate** - Check for broken references, warn if found
9. **Ready** - Graph ready for queries and edits

**Parallelization:** File parsing can happen across CPU cores. Each file parsed independently, then merged into single Graph.

**Performance target:** 1000 files in <1 second

### External File Edit

**Goal:** Import changes made in external editor (vim, VSCode, etc.)

**Trigger:** File watcher detects filesystem change

**Steps:**

1. **Verify not self-change** - Check for export lock (if present, we wrote it, ignore)
2. **Read new content** - Read markdown from filesystem
3. **Update CRDT** - Replace file content (operation logged)
4. **Parse file** - markdown → FlowNode AST → Nodes
5. **Extract IDs**:
   - Stable nodes: Extract from HTML comments
   - Temp nodes: Regenerate from position hashes
6. **Update Graph** - Remove old nodes for this file, add new nodes
7. **Update indices** - Rebuild by_file, by_tag, backlinks for affected nodes
8. **Update search** - Re-index stable nodes from this file
9. **Validate** - Check for broken references introduced by edit
10. **Sync** - CRDT operation automatically propagates to other devices

**Performance target:** <20ms for typical file

### User Creates Node

**Goal:** Add new node to space in Flow UI

**Steps:**

1. **Create in Graph** - Generate temp ID from file position, create node structure, add to parent's children
2. **Update indices** - Add to by_file (not by_tag yet, unpromoted)
3. **Render file** - Walk all nodes in file, render to markdown text (no HTML comment for temp ID)
4. **Update CRDT** - Replace file content (operation logged)
5. **Export (debounced)** - Mark file dirty, write to disk after delay
6. **No search index** - Temp nodes not indexed
7. **Sync** - CRDT operation propagates

**Performance target:** <10ms interactive response

### Create Reference (Promotion)

**Goal:** Reference one node from another, promoting temp to stable if needed

**Scenario:** User wants to reference "Task node" from another location

**Steps:**

1. **Check target** - Is it temp or stable?
2. **Promote if needed** - If temp:
   - Generate unique identifier
   - Update node ID in Graph (remove temp, re-insert as stable)
   - Update all indices
   - Mark node as promoted
3. **Add reference** - Insert reference syntax in source node text: `((n:abc123))`
4. **Update backlinks** - Add source to target's backlinks
5. **Render both files**:
   - Target file: Add HTML comment `<!-- n:abc123 -->` to node line
   - Source file: Contains reference `((n:abc123))`
6. **Update CRDT** - Both files updated (operations logged)
7. **Export** - Both files written to disk
8. **Index** - Add promoted node to search engine
9. **Sync** - Operations propagate

**Performance target:** <20ms

### Tag Node (Promotion)

**Goal:** Add tag to node, promoting if needed

**Steps:**

1. **Check node** - Is it temp or stable?
2. **Promote if temp**:
   - Generate unique identifier
   - Update Graph, indices
3. **Add tag** - Insert `#tagname` in node text
4. **Update indices** - Add to by_tag index
5. **Render file** - Include HTML comment for stable ID
6. **Update CRDT** - File content updated
7. **Export** - Write to disk
8. **Index** - Add to search engine with tag field
9. **Sync** - Propagate

**Performance target:** <15ms

### CLI Workflow (Temp IDs)

**Goal:** Query and manipulate nodes in command-line session

**Scenario:** User wants to process inbox items

**Example workflow:**

```bash
# Query returns position-based temp IDs
$ flow query "file:journals/today.md" --json
[
  {"id": "t:a3f821", "content": "Inbox item 1"},
  {"id": "t:b7k293", "content": "Inbox item 2"},
  {"id": "n:x9m4k1", "content": "Already tagged task", "tags": ["task"]}
]

# Edit using temp ID (works if no external changes)
$ flow edit "t:a3f821" --content "Updated inbox item"
{"id": "t:a3f821", "content": "Updated inbox item"}

# Tag promotes to stable ID
$ flow tag "t:b7k293" "task"
{"id": "n:c8h5j2", "content": "Inbox item 2", "tags": ["task"]}

# Move returns new temp ID (position changed)
$ flow move "t:a3f821" --to "projects/flow.md"
{"id": "t:k9m2n4", "location": "projects/flow.md:42:0"}
```

**Key behaviors:**
- Temp IDs valid for immediate operations in sequence
- Promotion returns new stable ID
- Move returns new temp ID (position changed)
- External edits invalidate temp IDs (re-query needed)

### Sync Between Devices

**Scenario:** Edit on Device A appears on Device B

**Device A:**
1. User edits → Graph updated
2. File rendered (includes HTML comments for stable IDs)
3. CRDT updated → operation generated
4. Operation sent to server
5. Server broadcasts to connected devices

**Device B:**
1. Receives CRDT operation
2. CRDT applies (automatic merge)
3. File content changed (subscriber notified)
4. Parse markdown:
   - Extract stable IDs from HTML comments
   - Regenerate temp IDs from positions
5. Rebuild Graph for file
6. Update indices
7. Update search index (stable nodes only)
8. Export to disk
9. UI refreshes

**Conflict example:**
- Device A & B both edit same line concurrently
- CRDT merges at character level
- HTML comments preserved (stable IDs intact)
- Both converge to same result
- No manual resolution needed

**Temp ID behavior:**
- Not synced (regenerated locally based on position)
- Same file position = same temp ID hash on all devices
- Unpromoted nodes remain unpromoted across sync

**Performance target:** <100ms total latency (edit to visible)

### Query Operations

**By tag:**
- Lookup in by_tag index: O(1)
- Return list of NodeIds (stable only, promoted nodes)
- Typical: <1ms

**Full-text search:**
- Parse query
- Search index (stable nodes only)
- Return stable node IDs
- Load nodes from Graph
- Typical: <10ms for 10k nodes

**Backlinks:**
- Lookup in backlinks index: O(1)
- Return list of NodeIds (stable only)
- Typical: <1ms

**Unlinked mentions:**
- Extract key terms from node text
- Build search query (OR of terms)
- Search index (stable nodes only)
- Filter out self, explicit backlinks, explicit references
- Typical: <10ms

**Complex queries:**
- Combine indices (e.g., by_tag + property filter)
- Iterator chaining and filtering
- Only stable nodes have tags/properties
- Typical: <5ms for moderate result sets

## Export System

**Goal:** Keep markdown files synchronized with CRDT state

**Debounced export strategy:**

Changes don't immediately write to disk. Instead:
1. Mark file as dirty in-memory set
2. Start/reset timer (e.g., 100ms)
3. When timer expires: Export all dirty files
4. Clear dirty set

**Benefits:**
- Batch multiple rapid edits to same file
- Reduce disk I/O
- Avoid thrashing external file watchers

**Per-file export lock:**
- Before writing a file, create lock file alongside it
- Prevents file watcher from treating our write as external edit
- Remove lock file after write completes
- Each file has its own lock (not a global space lock)

**Rendering process:**

For each dirty file:
1. Get all nodes for this file from Graph (by_file index)
2. Filter to root nodes (parent = None)
3. Recursively render tree:
   - Indentation: depth × 2 spaces
   - Bullet: `- `
   - Content text
   - If stable node: Append HTML comment `<!-- n:abc123 -->`
   - Other properties: `key:: value` lines (sorted alphabetically)
   - Tags: Already in content text
   - Recurse for children with depth + 1
4. Write to temp file
5. Atomic rename to target filename
6. Update filesystem mtime in CRDT metadata

**Atomic writes:** Temp file + rename ensures complete file or nothing, prevents corruption if crash during write.

**HTML Comment Placement:**
```markdown
- Task content here <!-- n:abc123 -->
  status:: done
  priority:: 5
  #project
```

Comment appears at end of first line for clean rendering.

## File Organization

**Space directory:**
```
my-space/
├─ .flow/
│  ├─ space.loro              # CRDT storage
│  ├─ search_index/           # Search index
│  └─ config.toml             # Space configuration
├─ journals/
│  ├─ 2024-11-30.md
│  └─ 2024-11-29.md
└─ pages/
   ├─ projects/
   │  └─ flow.md
   └─ meetings/
      └─ standup-notes.md
```

**Multi-space configuration:**
```
~/.flow/
├─ config.toml              # Global config
└─ spaces/
   ├─ personal/
   │  └─ .flow/
   └─ work/
      └─ .flow/
```

**File watching:**
- Watch journals/ and pages/ recursively
- Ignore .flow/ directory
- Ignore changes during export (check for lock)
- Debounce rapid changes (e.g., 100ms window)
- Cross-platform support required

## Error Handling & Recovery

### Corrupted CRDT File

**Detection:** Load fails with parse error

**Recovery strategy:**
1. Check .flow/backups/ for recent snapshots
2. If backup < 24h old: Restore from backup, warn user
3. If no recent backup: Rebuild from markdown
   - Scan all markdown files
   - Import each to new CRDT document
   - Extract stable IDs from HTML comments
   - CRDT history lost, content preserved
   - Warn: "Rebuilt from markdown, sync history lost"

### Corrupted Search Index

**Detection:** Search engine open/search fails

**Recovery:**
1. Delete search index directory
2. Rebuild from Graph (stable nodes only)
3. Index all promoted nodes
4. Fast operation (<5s typical)

### Markdown Parse Errors

**Detection:** Parser encounters malformed structure or custom syntax extraction fails

**Handling:**
- Log error with file path and approximate location
- Skip malformed content
- Continue parsing rest of file
- Show warning to user with file location
- Provide option to open file for manual fix

### Malformed HTML Comments

**Detection:** Invalid ID format in HTML comment

**Handling:**
- Log warning: "Invalid stable ID format in file:line"
- Treat node as unpromoted (generate temp ID)
- User can fix manually or re-promote

### Sync Conflicts

**Handling:** CRDT automatically resolves at character level

**Edge cases:**
- Concurrent delete and edit: CRDT may resurrect deleted content
- Multiple concurrent edits: Merged deterministically
- HTML comments preserved in merge (stable IDs intact)
- User notification: Show which files were merged from remote
- History: Option to view operation history
- Manual resolution: Only needed for semantic conflicts (CRDT can't understand meaning)

### Broken References

**Detection:** After parsing, validate all references point to existing nodes

**User experience:**
```bash
$ flow check
✗ 3 broken references found

File: journals/2024-11-30.md:42
  ((n:xyz789)) → Target does not exist
  
$ flow fix --interactive
Fix broken reference ((n:xyz789))?
  [1] Remove reference
  [2] Search for similar nodes
  [3] Skip
```

**Recovery via CRDT:**
If user broke references in external editor, they can restore to last working state using `flow restore`.

## Performance Targets

**Startup:**
- 1000 files, 50k nodes: <1 second
- Parallel parsing across CPU cores
- Typical: 8 cores parsing 125 files each

**Runtime operations:**
- Node creation: <10ms
- Node edit: <10ms
- Node promotion: <15ms
- File render: <10ms per file
- Tag query: <1ms
- Backlinks query: <1ms
- Full-text search: <10ms
- Unlinked mentions: <10ms

**Sync:**
- Local edit → remote visible: <50ms
- Remote edit → local visible: <20ms
- Conflict merge: automatic (no delay)

**Memory:**
- 50k nodes: ~70MB total
- 100k nodes: ~140MB total
- Acceptable for desktop/laptop use

**Disk:**
- CRDT file: ~10KB per 100 nodes
- Search index: ~50KB per 1000 stable nodes
- Markdown: User content size + HTML comments

## Security & Privacy

**Local-first guarantees:**
- All data stored locally by default
- No telemetry or analytics
- No required cloud services
- Sync server optional and self-hostable

**File permissions:**
- .flow/ directory: User read/write only
- Markdown files: Inherit parent directory permissions
- No elevation required
- Standard filesystem security

**Sync server (when enabled):**
- Optional end-to-end encryption (future)
- Self-hosted option
- Open source implementation
- User controls all data
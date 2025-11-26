# Flow Core Architecture Specification

## Architecture Overview

Flow uses an **event-sourced architecture** where Loro CRDTs serve as the single source of truth for all content mutations. Changes flow unidirectionally from Loro through subscriber systems that maintain derived state.

### Data Flow

```
User Action
    ↓
Loro Container Operations (Source of Truth)
    ↓
Change Events
    ↓ ↓ ↓
    ↓ ↓ └─→ [Extension Point: Plugin Hooks]
    ↓ └───→ [Indexer: Tantivy + SQLite]  
    └─────→ [Persister: Markdown Writer]
```

**Core Principle**: Loro receives all writes. Subscribers react to changes. All reads query derived indexes.

## Component Architecture

### 1. Loro Document Layer (Source of Truth)

**Responsibility**: Maintain authoritative CRDT state with conflict-free replication semantics.

**Structure**:
```
LoroDoc
├─ nodes: Map<NodeID, NodeContainer>
│  └─ <node-id>: Map
│     ├─ content: Text (Loro Text CRDT)
│     ├─ parent: String (NodeID or empty for root)
│     ├─ children: List (ordered NodeIDs)
│     ├─ created_at: i64
│     └─ modified_at: i64
│
└─ files: Map<FilePath, FileContainer>
   └─ <file-path>: Map
      └─ roots: List (root node IDs in file order)
```

**Operations**: Insert, update, delete nodes. All operations emit change events.

**Persistence**: `.flow/graph.loro` as binary snapshot. Configurable history retention window.

### 2. Index Layer (Query Engine)

**Responsibility**: Enable fast queries over graph structure, content, and metadata.

**Technology Stack**:
- **Tantivy**: Full-text search engine for content queries
- **SQLite**: Structured data for relations, hierarchy, and object model

#### Tantivy Index

**Purpose**: Full-text search across node content.

**Schema**:
```rust
Schema {
    id: String (STORED, INDEXED),      // NodeID
    content: Text (INDEXED),            // Searchable content
    file_path: String (STORED),         // For result display
}
```

**Operations**:
- `search(query: &str) -> Vec<NodeID>` - Full-text search with BM25 ranking
- `prefix_search(prefix: &str) -> Vec<NodeID>` - Auto-complete support

**Location**: `.flow/tantivy_index/`

#### SQLite Database

**Purpose**: Structured queries for graph relations and object model.

**Schema**:
```sql
-- Node metadata
CREATE TABLE nodes (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    file_path TEXT,
    parent_id TEXT,
    created_at INTEGER,
    modified_at INTEGER,
    FOREIGN KEY (parent_id) REFERENCES nodes(id)
);
CREATE INDEX idx_nodes_parent ON nodes(parent_id);
CREATE INDEX idx_nodes_file ON nodes(file_path);

-- Bidirectional references
CREATE TABLE references (
    source_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    target_text TEXT NOT NULL,
    PRIMARY KEY (source_id, target_id)
);
CREATE INDEX idx_references_target ON references(target_id);

-- Unresolved references (links to non-existent nodes)
CREATE TABLE unresolved_references (
    source_id TEXT NOT NULL,
    target_text TEXT NOT NULL,
    PRIMARY KEY (source_id, target_text)
);

-- Tag system
CREATE TABLE tags (
    node_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (node_id, tag)
);
CREATE INDEX idx_tags_tag ON tags(tag);

-- Property system (key::value inline syntax)
CREATE TABLE properties (
    node_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    value_type TEXT NOT NULL,  -- 'string' | 'number' | 'date' | 'reference'
    PRIMARY KEY (node_id, key)
);
CREATE INDEX idx_properties_key_value ON properties(key, value);

-- Version tracking for index invalidation
CREATE TABLE metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

**Operations**:
- `get_backlinks(node_id) -> Vec<NodeID>`
- `get_nodes_with_tag(tag) -> Vec<NodeID>`
- `query_by_property(key, value) -> Vec<NodeID>`
- `get_children(node_id) -> Vec<NodeID>` (ordered)

**Location**: `.flow/graph.db`

### 3. Subscriber System

**Pattern**: Observer pattern with synchronous execution in defined order.

#### Subscriber 1: Markdown Persister

**Trigger**: Node content changes, hierarchy changes.

**Behavior**:
1. Mark file as dirty when any node in file changes
2. On `save()`, reconstruct markdown from Loro containers
3. Walk node hierarchy to generate outline structure
4. Write to filesystem

**Batching**: Changes buffered until explicit `save()` call to avoid excessive I/O.

#### Subscriber 2: Index Updater

**Trigger**: All Loro change events.

**Behavior for content changes**:
1. Extract node content from Loro
2. Update Tantivy document (delete + reindex)
3. Update SQLite `nodes` table
4. Parse and update references (regex: `\[\[([^\]]+)\]\]`)
5. Parse and update tags (regex: `#([a-zA-Z0-9_-]+)`)
6. Parse and update properties (regex: `(\w+)::([^\s]+)`)

**Behavior for structure changes**:
1. Update parent/child relationships in SQLite
2. Update file mappings

**Reference Resolution**:
- Search SQLite for nodes matching `[[target_text]]`
- If found: Insert into `references` table
- If not found: Insert into `unresolved_references` table
- On any node content update: Retry all unresolved references

### 4. Markdown Layer (Presentation)

**Format**:
```markdown
- Root node content
  - Child node content
    - Grandchild node content
- Another root node
```

**Parsing**: On bootstrap, import existing markdown files into Loro.

**Generation**: Reconstruct markdown from Loro node hierarchy during save.

## Node Identity System

### ID Format: Nanoid

**Rationale**:
- **Compact**: 21 characters (default) vs UUID's 36
- **URL-safe**: `A-Za-z0-9_-` alphabet
- **CLI-friendly**: Short enough to type/remember
- **Customizable**: Configurable length and alphabet

**Generation**:
```rust
use nanoid::nanoid;

pub struct NodeId(String);

impl NodeId {
    pub fn new() -> Self {
        Self(nanoid!(12))  // 12 chars: ~180 years at 1000 IDs/hour
    }
}
```

**Collision Probability**: For 12-character Nanoid with default alphabet, ~1% collision chance after 100 million IDs.

**Properties**:
- Stable across edits
- Unique per node
- Independent of file location
- Never reused

## Graph Structure

```rust
pub struct Graph {
    path: PathBuf,
    metadata: GraphMetadata,
    
    // Source of truth
    document: LoroDoc,
    
    // Derived indexes
    tantivy_index: tantivy::Index,
    sqlite_db: rusqlite::Connection,
    
    // Dirty tracking
    dirty_files: HashSet<PathBuf>,
    
    // Subscribers
    subscriber_handles: Vec<SubscriptionId>,
}
```

## Initialization Sequence

### New Graph: `Graph::init(path, name)`

1. Create directory structure
2. Initialize empty Loro document
3. Create Tantivy index with schema
4. Create SQLite database with schema
5. Write metadata to `.flow/graph.toml`
6. Register subscribers
7. Perform initial save

### Load Existing: `Graph::load(path)`

1. Load metadata from `.flow/graph.toml`
2. Load Loro document from `.flow/graph.loro`
3. Open Tantivy index at `.flow/tantivy_index/`
4. Open SQLite database at `.flow/graph.db`
5. Check if indexes need rebuild (version mismatch)
6. If Loro empty but markdown exists: Bootstrap import
7. Register subscribers for incremental updates

## Markdown Import System

### Import Scenarios

**1. Bootstrap Import**: Initial setup with existing markdown files (Loro empty).

**2. Incremental Import**: Detect and import externally modified markdown files.

**3. Explicit Import**: User-triggered import via CLI command.

### Import Architecture

```
External Markdown Change
    ↓
Detection (File Watcher or Timestamp Check)
    ↓
Parse Markdown → Outline AST
    ↓
Pause Markdown Writer Subscriber
    ↓
Import to Loro Containers
    ↓
Index Subscriber Runs (build SQLite + Tantivy)
    ↓
Resume Markdown Writer Subscriber
```

**Critical**: Markdown writer subscriber must be paused during import to prevent circular write loops (import → subscriber writes file → triggers import → ...).

### Bootstrap Import Process

**Trigger**: `Graph::load()` detects Loro document empty but markdown files exist.

**Process**:
1. Scan directory tree for `.md` files
2. For each file:
   - Parse markdown into outline AST
   - Extract nested list structure
   - Create Loro nodes with generated NodeIDs
   - Build parent-child relationships
   - Add to file's roots list
3. Trigger full index rebuild from Loro state
4. Register subscribers for future changes

### Outline Parsing

**Input**: Markdown with nested lists.

```markdown
- Root node content
  - Child node content
    - Grandchild node content
  - Another child
- Second root node
```

**Output**: Hierarchical structure.

```rust
struct OutlineItem {
    content: String,
    children: Vec<OutlineItem>,
}
```

**Parser**: Use markdown parsing library (pulldown-cmark) to extract list items and their nesting depth.

### Import Implementation

```rust
impl Graph {
    fn import_markdown_file(&mut self, file_path: &Path) -> Result<()> {
        // Pause markdown writer to prevent circular updates
        self.pause_subscriber(SubscriberType::MarkdownWriter);
        
        let content = fs::read_to_string(file_path)?;
        let relative_path = file_path.strip_prefix(&self.path)?;
        
        // Parse outline structure
        let outline = parse_markdown_outline(&content)?;
        
        // Import into Loro recursively
        for item in outline {
            self.import_outline_item_recursive(item, None, relative_path)?;
        }
        
        // Resume markdown writer
        self.resume_subscriber(SubscriberType::MarkdownWriter);
        
        Ok(())
    }
    
    fn import_outline_item_recursive(
        &mut self,
        item: OutlineItem,
        parent_id: Option<NodeId>,
        file_path: &Path,
    ) -> Result<NodeId> {
        let node_id = NodeId::new();
        let nodes = self.document.get_map("nodes");
        let node = nodes.insert_container(node_id.to_string(), loro::ContainerType::Map)?;
        
        // Set content
        let content_text = node.insert_container("content", loro::ContainerType::Text)?;
        content_text.insert(0, &item.content)?;
        
        // Set parent
        node.insert("parent", parent_id.map(|id| id.to_string()).unwrap_or_default())?;
        
        // Create children list
        node.insert_container("children", loro::ContainerType::List)?;
        
        // Timestamps
        let now = chrono::Utc::now().timestamp();
        node.insert("created_at", now)?;
        node.insert("modified_at", now)?;
        
        // Add to parent's children or file roots
        if let Some(parent) = parent_id {
            let parent_node = nodes.get_map(parent.to_string());
            let children = parent_node.get_list("children");
            children.push(node_id.to_string())?;
        } else {
            let files = self.document.get_map("files");
            let file = files.get_or_create_map(file_path.to_str().unwrap());
            let roots = file.get_or_create_list("roots");
            roots.push(node_id.to_string())?;
        }
        
        // Recursively import children
        for child in item.children {
            self.import_outline_item_recursive(child, Some(node_id.clone()), file_path)?;
        }
        
        Ok(node_id)
    }
}
```

### Conflict Detection

**Problem**: Markdown file modified outside Flow while Loro contains different state.

**Detection Strategy**:

Compare file modification time with last Loro save time:
```rust
fn needs_import(&self, file_path: &Path) -> Result<bool> {
    let file_mtime = fs::metadata(file_path)?.modified()?;
    let loro_save_time = self.get_last_save_time()?;
    Ok(file_mtime > loro_save_time)
}
```

**Resolution Policies**:

1. **Loro-first (default)**: Ignore external changes. Loro is authoritative.
2. **Last-write-wins**: Import overwrites Loro state with file contents.
3. **Manual resolution**: Flag conflict, require explicit user choice.

**Recommendation**: Loro-first for synced graphs. Explicit import command for intentional overwrites.

### File Watching (Optional)

**Purpose**: Automatically detect and import externally modified files.

**Implementation**:
```rust
use notify::{Watcher, RecursiveMode, Event};

impl Graph {
    pub fn watch_files(&mut self) -> Result<()> {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    for path in event.paths {
                        if self.needs_import(&path)? {
                            self.import_markdown_file(&path)?;
                        }
                    }
                }
            }
        })?;
        
        watcher.watch(&self.path, RecursiveMode::Recursive)?;
        Ok(())
    }
}
```

**Tradeoffs**:
- **Pro**: Seamless external editor support (Vim, Emacs, VS Code)
- **Con**: Complexity, potential performance impact
- **Con**: Conflict potential in synced scenarios

**Recommendation**: Optional feature, disabled by default for synced graphs.

### CLI Commands

```bash
# Bootstrap import during initialization
flow init --import /path/to/existing/notes

# Import specific file (overwrites Loro state for that file)
flow import path/to/file.md

# Scan and import all new/modified files
flow import --scan

# Watch for file changes (daemon mode)
flow watch

# Check what would be imported (dry run)
flow import --scan --dry-run
```

### Import Ordering

Files imported in lexicographic order to ensure deterministic results and consistent NodeID assignment across multiple imports of same content.

### Idempotency

Importing same file twice should produce same Loro state (same node structure). Achieved by:
1. Clearing existing nodes for file before import
2. Regenerating NodeIDs deterministically (optional: use content hash for stable IDs)

**Alternative**: Track file → node mapping in SQLite to preserve NodeIDs across reimports.

## Query Interface

### Content Search
```rust
graph.search("machine learning") -> Vec<NodeId>
```
Uses Tantivy with BM25 ranking.

### Graph Traversal
```rust
graph.get_backlinks(node_id) -> Vec<NodeId>
graph.get_children(node_id) -> Vec<NodeId>
```
Uses SQLite for O(1) lookups.

### Object Model
```rust
graph.get_nodes_with_tag("project") -> Vec<NodeId>
graph.query_by_property("status", "active") -> Vec<NodeId>
```
Uses SQLite indexes.

## Inline Syntax Parsing

### References: `[[target]]`
- Regex: `\[\[([^\]]+)\]\]`
- Extracted during index updates
- Resolved by searching node content in SQLite
- Stored in `references` or `unresolved_references`

### Tags: `#tag-name`
- Regex: `#([a-zA-Z0-9_-]+)`
- Extracted during index updates
- Stored in `tags` table
- Powers `get_nodes_with_tag()` queries

### Properties: `key::value`
- Regex: `(\w+)::([^\s]+)`
- Extracted during index updates
- Type inferred: number, reference (if `[[...]]`), else string
- Stored in `properties` table
- Powers object model queries

## History Management

### Loro Compaction

**Problem**: Unbounded operation history grows file size.

**Solution**: Configurable retention window.

```rust
impl Graph {
    pub fn compact(&mut self, retain_days: u32) -> Result<()> {
        let snapshot = self.document.export(ExportMode::Snapshot)?;
        self.document = LoroDoc::new();
        self.document.import(&snapshot)?;
        self.save()
    }
}
```

**Trigger Options**:
- Operation count threshold
- File size threshold
- Time-based (user-configured retention window)
- Manual command

## Technology Justification

### Tantivy over SQLite FTS5
- **Performance**: 2x faster than Lucene, significantly faster than SQLite FTS5
- **Features**: Advanced ranking (BM25), phrase queries, fuzzy search
- **Scalability**: Designed for large document collections
- **Rust-native**: Zero FFI overhead, type-safe API

### SQLite for Structured Data
- **Maturity**: Battle-tested, well-understood failure modes
- **ACID**: Guaranteed consistency for graph relations
- **Query power**: Complex joins, aggregations for object model
- **Portability**: Standard format, inspectable with CLI tools

### Nanoid over UUID
- **Brevity**: 12 chars vs 36 (66% reduction)
- **Readability**: Avoids visual confusion (no 0/O, 1/l)
- **CLI ergonomics**: Typeable, memorable for short sessions
- **Security**: Cryptographically secure random generation

### Loro over Operational Transform
- **CRDT guarantees**: No central authority needed for sync
- **Conflict-free**: Automatic merge without user intervention
- **Mature**: Production-ready library with active development
- **Rust-native**: Performance and type safety

## Future Extensions

### Plugin Hooks
Third subscriber slot available for custom logic:
- Export integrations (Notion, Obsidian)
- AI embeddings generation
- Custom indexing strategies
- Analytics and metrics

### Meta-Model System
Tag definitions as nodes (future phase):
```markdown
- #tag-definition Status
  - property::name "Status"
  - property::values ["active", "completed", "archived"]
  - property::color "#ff0000"
```

Enables Tana-like self-hosted type system.

## Performance Targets

- Node creation: <1ms
- Full-text search: <50ms for 100k nodes
- Backlink queries: <5ms
- Graph load: <100ms for 10k nodes
- Save operation: <500ms for 100 dirty files

## Crate Dependencies

```toml
[dependencies]
loro = "1.0"
tantivy = "0.22"
rusqlite = { version = "0.32", features = ["bundled"] }
nanoid = "0.4"
regex = "1.10"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
chrono = "0.4"
miette = "7.0"
walkdir = "2.4"
pulldown-cmark = "0.11"  # Markdown parsing
notify = "6.1"  # File watching (optional feature)
```

## File Structure

```
<graph-path>/
├─ .flow/
│  ├─ graph.toml           # Metadata
│  ├─ graph.loro           # CRDT state
│  ├─ graph.db             # SQLite index
│  └─ tantivy_index/       # Tantivy segments
├─ journal/
│  └─ YYYY-MM-DD.md
└─ *.md                    # User markdown files
```

## Implementation Phases

1. **Core infrastructure**: Loro integration, node CRUD, save/load
2. **Index layer**: Tantivy + SQLite setup, subscriber system
3. **Markdown import/export**: Parse markdown, bootstrap import, file generation
4. **Inline parsing**: Reference, tag, property extraction
5. **Query interface**: Search, backlinks, object model queries
6. **History management**: Compaction, retention policies
7. **File watching** (optional): Auto-import external changes

## Open Questions

1. **Multi-file references**: How to handle nodes split across files?
2. **Schema migration**: Strategy for index schema changes?
3. **Large files**: Performance threshold for splitting daily notes?
4. **Sync conflicts**: UI for presenting concurrent edits?

---

This specification defines a robust architecture that separates concerns cleanly: Loro for authoritative state, indexes for queries, markdown for human readability. The subscriber pattern enables future extensibility while maintaining architectural clarity.

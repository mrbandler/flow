# Flow Core Module Specification

## Overview

The core module provides the fundamental graph operations and data management layer. All frontends (CLI, TUI, desktop GUI, web) interact with the graph exclusively through this module's API.

**Responsibilities:**
- Node lifecycle management (CRUD)
- Reference tracking and resolution
- Tag and property management
- Query execution
- Index maintenance
- Persistence and serialization
- CRDT synchronization primitives
- Schema validation

**Non-Responsibilities:**
- User interface rendering
- Input handling
- Command parsing
- Network communication (handled by server module)

---

## Architecture Layers

```
┌─────────────────────────────────────┐
│   Frontends (CLI, TUI, GUI, Web)    │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│          Core Module API            │
│  (Public interface for all clients) │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│         Core Systems                │
│  ┌──────────────────────────────┐  │
│  │  Graph State                 │  │
│  │  - Nodes, Tags, Properties   │  │
│  │  - Reference Graph           │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Index System                │  │
│  │  - Tag Index                 │  │
│  │  - Property Index            │  │
│  │  - Full-text Index           │  │
│  │  - Reference Index           │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Query Engine                │  │
│  │  - Expression Parser         │  │
│  │  - Filter Executor           │  │
│  │  - Result Ranking            │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Validation System           │  │
│  │  - Schema Validation         │  │
│  │  - Type Checking             │  │
│  │  - Referential Integrity     │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Persistence Layer           │  │
│  │  - Markdown I/O              │  │
│  │  - Loro CRDT Integration     │  │
│  │  - Transaction Management    │  │
│  └──────────────────────────────┘  │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│          Filesystem                 │
│  - Markdown files                   │
│  - .flow/ metadata                  │
│  - Loro container state             │
└─────────────────────────────────────┘
```

---

## Core Data Structures

### Node

Primary entity in the graph. Every piece of content is a node.

```rust
struct Node {
    id: NodeId,              // UUID v4
    content: String,         // Markdown content (cleaned of inline syntax)
    children: Vec<NodeId>,   // Child node IDs (ordered)
    parent: Option<NodeId>,  // Parent node ID
    tags: Vec<String>,       // Tag names applied to this node
    properties: PropertyMap, // Key-value properties
    references: ReferenceSet, // Outgoing and incoming references
    metadata: NodeMetadata,  // Created, modified, author, etc.
    crdt_version: u64,       // Loro version vector
}

type NodeId = Uuid;

struct NodeMetadata {
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    author: String,
    file_path: Option<PathBuf>, // Source markdown file
}
```

### Property

Typed key-value pair attached to nodes.

```rust
struct Property {
    key: String,
    value: PropertyValue,
    property_type: PropertyType,
}

enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Date(DateTime<Utc>),
    Reference(NodeId),
    List(Vec<PropertyValue>),
}

enum PropertyType {
    String,
    Number,
    Boolean,
    Date,
    Reference,
    List(Box<PropertyType>),
    Enum(Vec<String>), // Constrained to specific values
}

type PropertyMap = HashMap<String, Property>;
```

### Reference

Bidirectional link between nodes.

```rust
struct Reference {
    source: NodeId,
    target: NodeId,
    reference_type: ReferenceType,
    created: DateTime<Utc>,
}

enum ReferenceType {
    Explicit,      // User-created via flow link
    Implicit,      // Extracted from content (wiki-links, etc.)
    Hierarchical,  // Parent-child relationship
}

struct ReferenceSet {
    outgoing: Vec<Reference>,
    incoming: Vec<Reference>,
}
```

### Tag

Marker that converts nodes into objects with schema.

```rust
struct Tag {
    name: String,
    definition: Option<NodeId>, // Points to #tag-definition node
    color: Option<String>,
    icon: Option<String>,
}
```

### Graph

Root container for all graph state.

```rust
struct Graph {
    id: Uuid,
    name: String,
    path: PathBuf,
    nodes: HashMap<NodeId, Node>,
    tags: HashMap<String, Tag>,
    indexes: IndexSystem,
    loro_doc: LoroDoc,
    dirty_nodes: HashSet<NodeId>,
}
```

---

## Node Management

### Node Creation

```rust
pub fn create_node(
    graph: &mut Graph,
    content: &str,
    parent: Option<NodeId>,
    tags: Vec<String>,
    properties: PropertyMap,
) -> Result<NodeId, GraphError>
```

**Process:**
1. Generate new UUID
2. Parse inline syntax from content (#tags, key::value)
3. Merge explicit tags/properties with parsed ones
4. Create Node struct
5. Update parent's children list
6. Add to graph.nodes
7. Update indexes
8. Create Loro operations
9. Mark as dirty
10. Return NodeId

### Node Retrieval

```rust
pub fn get_node(graph: &Graph, id: NodeId) -> Result<&Node, GraphError>
pub fn get_node_mut(graph: &mut Graph, id: NodeId) -> Result<&mut Node, GraphError>
```

### Node Update

```rust
pub fn update_node_content(
    graph: &mut Graph,
    id: NodeId,
    new_content: &str,
) -> Result<(), GraphError>

pub fn update_node_properties(
    graph: &mut Graph,
    id: NodeId,
    properties: PropertyMap,
) -> Result<(), GraphError>
```

**Process:**
1. Validate node exists
2. Parse inline syntax if updating content
3. Update Node struct
4. Update modified timestamp
5. Trigger index updates
6. Create Loro operations
7. Mark as dirty
8. Run validation

### Node Deletion

```rust
pub fn delete_node(
    graph: &mut Graph,
    id: NodeId,
    recursive: bool,
    reparent_children: bool,
) -> Result<(), GraphError>
```

**Process:**
1. Validate node exists
2. If recursive, delete all descendants
3. If reparent_children, move children to deleted node's parent
4. Remove from parent's children list
5. Delete all references to/from node
6. Remove from indexes
7. Remove from graph.nodes
8. Create Loro deletion operations
9. Delete markdown file if exists

### Node Hierarchy

```rust
pub fn move_node(
    graph: &mut Graph,
    node_id: NodeId,
    new_parent: NodeId,
    position: Option<usize>,
) -> Result<(), GraphError>

pub fn get_children(graph: &Graph, node_id: NodeId) -> Vec<NodeId>
pub fn get_parent(graph: &Graph, node_id: NodeId) -> Option<NodeId>
pub fn get_ancestors(graph: &Graph, node_id: NodeId) -> Vec<NodeId>
pub fn get_descendants(graph: &Graph, node_id: NodeId) -> Vec<NodeId>
```

---

## Tag Management

```rust
pub fn apply_tag(
    graph: &mut Graph,
    node_id: NodeId,
    tag: &str,
) -> Result<(), GraphError>

pub fn remove_tag(
    graph: &mut Graph,
    node_id: NodeId,
    tag: &str,
) -> Result<(), GraphError>

pub fn get_nodes_with_tag(graph: &Graph, tag: &str) -> Vec<NodeId>

pub fn list_all_tags(graph: &Graph) -> Vec<String>
```

**Tag Index:**
```rust
struct TagIndex {
    tag_to_nodes: HashMap<String, HashSet<NodeId>>,
    node_to_tags: HashMap<NodeId, HashSet<String>>,
}
```

### Tag Templates

Tags can include template structures. When a tag definition node has children, those children are treated as a template.

```rust
pub fn get_tag_template(
    graph: &Graph,
    tag: &str,
) -> Option<Vec<NodeId>>

pub fn has_template(
    graph: &Graph,
    tag: &str,
) -> bool

pub fn apply_tag_with_template(
    graph: &mut Graph,
    node_id: NodeId,
    tag: &str,
) -> Result<(), GraphError>
```

**Template Application Process:**

1. Apply the tag to the node
2. Look up tag definition node by querying for `'tag-definition' IN tags AND name = '<tag>'`
3. Get all children of the tag definition node
4. Deep copy each child node recursively
5. Add copies as children of the target node
6. Preserve structure and properties from template

**Example:**

```rust
// Tag definition with template
let tag_def_id = create_tag_definition(graph, "project", vec!["status"])?;
let goals_id = create_node(graph, "## Goals", Some(tag_def_id), vec![], HashMap::new())?;
let milestones_id = create_node(graph, "## Milestones", Some(tag_def_id), vec![], HashMap::new())?;

// Apply tag with template to a node
let node_id = create_node(graph, "New Project", None, vec![], HashMap::new())?;
apply_tag_with_template(graph, node_id, "project")?;

// Node now has "## Goals" and "## Milestones" as children
assert_eq!(get_children(graph, node_id).len(), 2);
```

---

## Property Management

```rust
pub fn set_property(
    graph: &mut Graph,
    node_id: NodeId,
    key: &str,
    value: PropertyValue,
) -> Result<(), GraphError>

pub fn get_property(
    graph: &Graph,
    node_id: NodeId,
    key: &str,
) -> Option<&Property>

pub fn delete_property(
    graph: &mut Graph,
    node_id: NodeId,
    key: &str,
) -> Result<(), GraphError>

pub fn get_nodes_by_property(
    graph: &Graph,
    key: &str,
    value: &PropertyValue,
) -> Vec<NodeId>
```

**Property Index:**
```rust
struct PropertyIndex {
    // key -> value -> nodes
    property_values: HashMap<String, HashMap<PropertyValue, HashSet<NodeId>>>,
    // node -> properties
    node_properties: HashMap<NodeId, PropertyMap>,
}
```

---

## Reference Management

### Explicit References

User-created links between nodes.

```rust
pub fn create_reference(
    graph: &mut Graph,
    source: NodeId,
    target: NodeId,
) -> Result<(), GraphError>

pub fn delete_reference(
    graph: &mut Graph,
    source: NodeId,
    target: NodeId,
) -> Result<(), GraphError>

pub fn get_outgoing_references(graph: &Graph, node_id: NodeId) -> Vec<Reference>
pub fn get_incoming_references(graph: &Graph, node_id: NodeId) -> Vec<Reference>
pub fn get_all_references(graph: &Graph, node_id: NodeId) -> ReferenceSet
```

### Implicit References

Extracted from content (wiki-links, @mentions, etc.).

```rust
pub fn extract_implicit_references(content: &str) -> Vec<NodeId>
pub fn update_implicit_references(graph: &mut Graph, node_id: NodeId) -> Result<(), GraphError>
```

**Reference Patterns:**
- `[[node-id]]` - Wiki-link to node
- `@node-id` - Mention/reference
- Extracted during content updates

**Reference Index:**
```rust
struct ReferenceIndex {
    outgoing: HashMap<NodeId, HashSet<NodeId>>,
    incoming: HashMap<NodeId, HashSet<NodeId>>,
    // Reference graph for path finding
    graph: petgraph::Graph<NodeId, ()>,
}
```

### Graph Traversal

```rust
pub fn find_path(
    graph: &Graph,
    from: NodeId,
    to: NodeId,
) -> Option<Vec<NodeId>>

pub fn find_related_nodes(
    graph: &Graph,
    node_id: NodeId,
    depth: usize,
) -> Vec<NodeId>

pub fn get_connected_component(
    graph: &Graph,
    node_id: NodeId,
) -> Vec<NodeId>
```

---

## Query Engine

### SQL Query Language

Flow uses SQL syntax for querying the graph. Nodes are treated as rows in a virtual `nodes` table.

**Virtual Schema:**

```sql
TABLE nodes (
    id TEXT PRIMARY KEY,
    content TEXT,
    created TIMESTAMP,
    modified TIMESTAMP,
    author TEXT,
    tags TEXT[],              -- Array of tag names
    parent_id TEXT,
    -- All properties flattened as columns
    -- e.g., status, priority, owner, etc.
)
```

**Supported SQL Features:**

```sql
-- Basic selection
SELECT * FROM nodes WHERE 'project' IN tags

-- Property filtering
SELECT * FROM nodes WHERE status = 'active' AND priority > 3

-- Date comparisons
SELECT * FROM nodes WHERE created > '2024-11-01'
SELECT * FROM nodes WHERE modified < CURRENT_DATE - INTERVAL '7 days'

-- Content search
SELECT * FROM nodes WHERE content LIKE '%CRDT%'
SELECT * FROM nodes WHERE content ~ 'regex pattern'

-- Multiple tags
SELECT * FROM nodes WHERE 'project' IN tags AND 'active' IN tags

-- Combining conditions
SELECT * FROM nodes 
WHERE ('task' IN tags OR 'bug' IN tags) 
  AND status != 'done' 
  AND priority >= 4

-- Sorting
SELECT * FROM nodes WHERE 'project' IN tags ORDER BY priority DESC, created ASC

-- Limiting results
SELECT * FROM nodes WHERE 'article' IN tags LIMIT 10

-- Hierarchical queries
SELECT * FROM nodes WHERE parent_id = 'abc-123-def'

-- Date references work in queries
SELECT * FROM nodes WHERE created >= '@-7d'
```

**Special Query Features:**

```sql
-- Tag existence
'tag-name' IN tags

-- Property existence
property_name IS NOT NULL

-- Array/list properties
'item' IN property_list

-- Reference properties
owner = '@node-id'

-- Current date/time
CURRENT_DATE
CURRENT_TIMESTAMP
NOW()

-- Date arithmetic
created > CURRENT_DATE - INTERVAL '7 days'
due_date < '@today'
```

### Query Execution

```rust
pub struct SqlQuery {
    ast: sqlparser::ast::Statement,
}

pub fn parse_sql_query(sql: &str) -> Result<SqlQuery, ParseError>

pub fn execute_sql_query(
    graph: &Graph,
    query: &SqlQuery,
) -> Result<Vec<NodeId>, GraphError>
```

**Execution Strategy:**
1. Parse SQL string into AST using `sqlparser` crate
2. Extract WHERE clause conditions
3. For each condition, use appropriate index:
   - Tag filters (`'tag' IN tags`) → TagIndex
   - Property filters (`prop = value`) → PropertyIndex
   - Content filters (`content LIKE '%text%'`) → FullTextIndex
   - Hierarchical filters (`parent_id = 'id'`) → Reference lookups
4. Combine results using set operations (AND, OR, NOT)
5. Apply ORDER BY, LIMIT, OFFSET
6. Return ordered NodeId list

**Index Utilization:**

Query planner automatically uses indexes:

```sql
-- Uses TagIndex
WHERE 'project' IN tags

-- Uses PropertyIndex  
WHERE status = 'active'

-- Uses FullTextIndex
WHERE content LIKE '%search term%'

-- Uses multiple indexes, combines with AND
WHERE 'task' IN tags AND priority > 3

-- Full scan (no applicable index)
WHERE LENGTH(content) > 1000
```

---

## Index System

### Full-Text Search Index

```rust
struct FullTextIndex {
    // Inverted index: term -> nodes containing term
    term_index: HashMap<String, HashSet<NodeId>>,
    // Node -> terms (for updates)
    node_terms: HashMap<NodeId, HashSet<String>>,
}

pub fn index_node_content(index: &mut FullTextIndex, node: &Node)
pub fn search_content(index: &FullTextIndex, query: &str) -> Vec<NodeId>
pub fn remove_from_index(index: &mut FullTextIndex, node_id: NodeId)
```

**Tokenization:**
- Lowercase normalization
- Stop word removal (optional)
- Stemming (optional)
- Support for CJK languages
- Preserve markdown syntax awareness

### Index Maintenance

All indexes updated automatically on node mutations:

```rust
pub fn update_indexes(
    graph: &mut Graph,
    node_id: NodeId,
    operation: IndexOperation,
)

enum IndexOperation {
    Insert,
    Update,
    Delete,
}
```

**Update triggers:**
- Node creation → Insert into all indexes
- Node content change → Update FullTextIndex
- Tag add/remove → Update TagIndex
- Property set/delete → Update PropertyIndex
- Reference create/delete → Update ReferenceIndex
- Node deletion → Remove from all indexes

---

## Persistence Layer

### Storage Structure

```
graph_root/
├── .flow/
│   ├── graph.toml              # Graph metadata
│   ├── loro.bin                # CRDT state
│   ├── context                 # Last accessed node
│   └── indexes/                # Serialized indexes (optional)
├── journal/
│   ├── 2024-11-20.md
│   ├── 2024-11-21.md
│   └── 2024-11-24.md
└── nodes/
    ├── abc-123-def.md
    ├── xyz-789-abc.md
    └── ...
```

### Markdown File Format

```markdown
---
id: abc-123-def
created: 2024-11-20T14:22:00Z
modified: 2024-11-24T10:15:00Z
tags: [project, active]
---

# Project: Flow CLI

This is the content with inline properties like status::planning.

Child nodes are represented as nested list items:
- First child node
  - Grandchild node
- Second child node

References: [[xyz-789-abc]], [[def-456-ghi]]
```

### Persistence Operations

```rust
pub fn load_graph(path: &Path) -> Result<Graph, GraphError>
pub fn save_graph(graph: &Graph) -> Result<(), GraphError>
pub fn flush_dirty_nodes(graph: &mut Graph) -> Result<(), GraphError>

// Node-level operations
pub fn load_node_from_file(path: &Path) -> Result<Node, GraphError>
pub fn save_node_to_file(node: &Node, path: &Path) -> Result<(), GraphError>
```

**Load Process:**
1. Read graph metadata from `.flow/graph.toml`
2. Load Loro container from `.flow/loro.bin`
3. Scan `journal/` and `nodes/` directories
4. Parse markdown files into Node structs
5. Build indexes
6. Reconstruct reference graph
7. Validate referential integrity

**Save Process:**
1. Flush dirty nodes to markdown files
2. Serialize Loro container to `.flow/loro.bin`
3. Update graph metadata
4. Optionally serialize indexes for faster loading

### Dirty Tracking

```rust
struct DirtySet {
    nodes: HashSet<NodeId>,
}

pub fn mark_dirty(graph: &mut Graph, node_id: NodeId)
pub fn flush_dirty(graph: &mut Graph) -> Result<(), GraphError>
```

Only modified nodes written to disk. Tracking prevents unnecessary I/O.

---

## CRDT Integration (Loro)

### Loro Document Structure

```rust
struct LoroGraph {
    doc: LoroDoc,
    // Map: node_id -> LoroMap
    nodes: LoroMap,
}
```

Each node stored as LoroMap with fields:
- `content` → LoroText (for collaborative editing)
- `tags` → LoroList
- `properties` → LoroMap
- `children` → LoroList
- `metadata` → LoroMap

### Operation Capture

Every mutation creates Loro operations:

```rust
pub fn apply_loro_operations(graph: &mut Graph, ops: &[LoroOp]) -> Result<(), GraphError>
pub fn export_loro_operations(graph: &Graph) -> Vec<LoroOp>
```

**Sync workflow:**
1. Local changes create Loro operations
2. Operations batched and exported
3. Sent to sync server
4. Remote operations imported
5. Loro handles conflict resolution
6. Graph state updated from Loro doc

---

## Validation System

### Schema Validation

```rust
pub fn validate_node(graph: &Graph, node_id: NodeId) -> Vec<ValidationError>
pub fn validate_graph(graph: &Graph) -> Vec<ValidationError>

pub struct ValidationError {
    node_id: NodeId,
    error_type: ValidationErrorType,
    message: String,
}

pub enum ValidationErrorType {
    MissingRequiredProperty,
    InvalidPropertyType,
    InvalidPropertyValue,
    InvalidReference,
    SchemaViolation,
    OrphanedNode,
}
```

**Validation Checks:**
1. Tag schema compliance
2. Required properties present
3. Property types match schema
4. Property values within constraints
5. References point to existing nodes
6. Hierarchical integrity (no cycles)

### Referential Integrity

```rust
pub fn check_referential_integrity(graph: &Graph) -> Vec<ValidationError>
```

**Checks:**
- All parent IDs point to existing nodes
- All child IDs point to existing nodes
- All references point to existing nodes
- No orphaned nodes (except root nodes)
- No cycles in hierarchical parent-child relationships

---

## Transaction Management

```rust
pub struct Transaction<'a> {
    graph: &'a mut Graph,
    operations: Vec<Operation>,
    rollback_state: Option<GraphSnapshot>,
}

impl Transaction<'_> {
    pub fn begin(graph: &mut Graph) -> Self
    pub fn commit(self) -> Result<(), GraphError>
    pub fn rollback(self) -> Result<(), GraphError>
}

pub fn with_transaction<F, R>(graph: &mut Graph, f: F) -> Result<R, GraphError>
where
    F: FnOnce(&mut Transaction) -> Result<R, GraphError>
```

**Usage:**
```rust
with_transaction(graph, |tx| {
    let node_id = create_node(tx.graph, content, None, vec![], HashMap::new())?;
    apply_tag(tx.graph, node_id, "project")?;
    set_property(tx.graph, node_id, "status", PropertyValue::String("active".into()))?;
    Ok(node_id)
})?;
```

All operations in transaction block are atomic. Rollback on any error.

---

## Date References

Special node type for journal entries:

```rust
pub fn get_journal_node(graph: &Graph, date: NaiveDate) -> Option<NodeId>
pub fn create_journal_node(graph: &mut Graph, date: NaiveDate) -> Result<NodeId, GraphError>

pub fn parse_date_reference(expr: &str) -> Result<NaiveDate, ParseError>
```

**Date expressions:**
- `@today`, `@yesterday`, `@tomorrow`
- `@2024-11-24`
- `@-3d`, `@+1w`, `@-2m`

Journal nodes automatically created in `journal/` directory with filename `YYYY-MM-DD.md`.

---

## Error Handling

```rust
pub enum GraphError {
    NodeNotFound(NodeId),
    InvalidReference(NodeId, NodeId),
    ValidationError(Vec<ValidationError>),
    CyclicHierarchy(NodeId),
    IoError(std::io::Error),
    LoroError(loro::Error),
    ParseError(String),
    SchemaViolation(String),
}

pub type Result<T> = std::result::Result<T, GraphError>;
```

All public API functions return `Result<T, GraphError>` for explicit error handling.

---

## Public API Surface

Core module exposes these categories of operations:

### Graph Management
- `load_graph`, `save_graph`, `flush_dirty_nodes`

### Node Operations
- `create_node`, `get_node`, `update_node_content`, `delete_node`
- `move_node`, `get_children`, `get_parent`, `get_ancestors`, `get_descendants`

### Tag Operations
- `apply_tag`, `remove_tag`, `get_nodes_with_tag`, `list_all_tags`
- `get_tag_template`, `apply_tag_with_template`, `has_template`

### Property Operations
- `set_property`, `get_property`, `delete_property`, `get_nodes_by_property`

### Reference Operations
- `create_reference`, `delete_reference`
- `get_outgoing_references`, `get_incoming_references`, `get_all_references`
- `find_path`, `find_related_nodes`

### Query Operations
- `parse_sql_query`, `execute_sql_query`, `search_content`

### Validation
- `validate_node`, `validate_graph`, `check_referential_integrity`

### Date Operations
- `get_journal_node`, `create_journal_node`, `parse_date_reference`

### Transaction Operations
- `Transaction::begin`, `Transaction::commit`, `Transaction::rollback`
- `with_transaction`

---

## Performance Considerations

### Lazy Loading

Large graphs require lazy node loading:

```rust
pub struct LazyGraph {
    metadata: GraphMetadata,
    loaded_nodes: HashMap<NodeId, Node>,
    node_paths: HashMap<NodeId, PathBuf>,
}

pub fn load_node_on_demand(graph: &mut LazyGraph, node_id: NodeId) -> Result<&Node, GraphError>
```

Only load nodes into memory when accessed. Cache hot nodes.

### Index Persistence

Large graphs benefit from persisted indexes:

```rust
pub fn serialize_indexes(graph: &Graph, path: &Path) -> Result<(), GraphError>
pub fn deserialize_indexes(path: &Path) -> Result<IndexSystem, GraphError>
```

Rebuilding indexes on every load expensive for large graphs. Serialize to disk.

### Incremental Updates

Loro CRDT provides incremental sync. Avoid full graph serialization:

```rust
pub fn export_updates_since(graph: &Graph, version: u64) -> Vec<LoroOp>
pub fn import_updates(graph: &mut Graph, ops: Vec<LoroOp>) -> Result<(), GraphError>
```

---

## Testing Requirements

Core module requires comprehensive test coverage:

1. **Unit Tests**: Every public function
2. **Property Tests**: Invariants (no cycles, referential integrity)
3. **Integration Tests**: Full workflows (create -> update -> query -> delete)
4. **Benchmark Tests**: Performance regression prevention
5. **CRDT Tests**: Merge scenarios, conflict resolution
6. **Persistence Tests**: Save/load round-trips, corruption recovery

---

## Future Extensions

### Multi-Graph Operations

Support operations across multiple graphs:

```rust
pub fn merge_graphs(graph1: &Graph, graph2: &Graph) -> Result<Graph, GraphError>
pub fn copy_node_between_graphs(
    source: &Graph,
    target: &mut Graph,
    node_id: NodeId,
) -> Result<NodeId, GraphError>
```

### Advanced Query Features

- Regex support in content queries
- Fuzzy date ranges
- Geographic/spatial queries (if lat/lon properties)
- Graph pattern matching (subgraph isomorphism)

### Optimization

- Bloom filters for negative lookups
- Parallel index updates
- Memory-mapped file access for large graphs
- Incremental full-text indexing

---

## Summary

Core module provides complete graph management with:
- Node CRUD with hierarchical relationships
- Bidirectional reference tracking
- Tag and property system
- Powerful query engine with multiple indexes
- Markdown-based persistence with CRDT integration
- Schema validation and referential integrity
- Transaction support for atomic operations

All frontends interact exclusively through this API, ensuring consistent behavior across CLI, TUI, desktop GUI, and web interfaces.
# Flow Meta-Model Specification

## Overview

Flow implements a self-hosting meta-model where the type system itself exists as nodes within the graph. This allows users to define custom tags (object types) and properties with schemas, validation rules, and constraints—all using the same primitives available for regular content.

**Core Principle:** Everything is a node. Tags and properties are defined using special built-in tags that create definitions stored as nodes in the graph.

---

## Built-in Primitive Tags

These tags are hardcoded into the system and enable users to build custom schemas.

### `#tag-definition`

Defines a new tag (object type) that can be applied to nodes. Child nodes of the tag definition act as a template, automatically added when the tag is applied.

**Purpose:** Creates a custom tag with associated schema, appearance, behavior, and content template.

**Built-in Properties:**

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `name` | string | Yes | Unique identifier for the tag |
| `description` | string | No | Human-readable description |
| `color` | string | No | Hex color code for UI display |
| `icon` | string | No | Unicode emoji or icon |
| `schema` | list | No | List of property names this tag expects |
| `required-properties` | list | No | Properties that must be present |
| `exclusive` | boolean | No | Only one instance of this tag per node |

**Template Behavior:**

Any child nodes of the tag definition are treated as a template. When the tag is applied to a node, the template children are copied under the tagged node.

**Example:**

```bash
flow create "Project Tag #tag-definition 
  name::project 
  description::\"Represents a project with status tracking\" 
  color::#3b82f6 
  icon::📦 
  schema::[status,priority,owner,due-date,description] 
  required-properties::[status]"

# Add template structure as children
flow append <project-tag-def-id> "## Goals"
flow append <project-tag-def-id> "## Milestones"
flow append <project-tag-def-id> "## Resources"
flow append <project-tag-def-id> "## Notes"
```

**Resulting Tag Definition Node:**
```markdown
---
id: tag-def-001
tags: [tag-definition]
---

# Project Tag

name:: project
description:: Represents a project with status tracking
color:: #3b82f6
icon:: 📦
schema:: [status, priority, owner, due-date, description]
required-properties:: [status]

- ## Goals
- ## Milestones
- ## Resources
- ## Notes
```

**When Applied:**

```bash
flow create "New Product Launch #project status::planning"
```

The system automatically creates:
```markdown
# New Product Launch

status:: planning

- ## Goals
- ## Milestones
- ## Resources
- ## Notes
```

### `#property-definition`

Defines a property with type, constraints, and validation rules.

**Purpose:** Creates a reusable property that can be applied to any node, with type safety and validation.

**Built-in Properties:**

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `name` | string | Yes | Property key identifier |
| `type` | enum | Yes | Data type (string, number, boolean, date, reference, list, enum) |
| `description` | string | No | Human-readable description |
| `required` | boolean | No | Must be present (default: false) |
| `default` | any | No | Default value if not specified |
| `values` | list | No | For enum type: allowed values |
| `min` | number | No | For number type: minimum value |
| `max` | number | No | For number type: maximum value |
| `pattern` | string | No | For string type: regex validation pattern |
| `reference-tag` | string | No | For reference type: target must have this tag |
| `multi` | boolean | No | Allow multiple values (converts to list) |

**Example:**

```bash
flow create "Status Property #property-definition 
  name::status 
  type::enum 
  values::[planning,active,blocked,done,archived] 
  required::false 
  default::planning 
  description::\"Current state of a task or project\""
```

**Resulting Node:**
```markdown
---
id: prop-def-001
tags: [property-definition]
---

# Status Property

name:: status
type:: enum
values:: [planning, active, blocked, done, archived]
required:: false
default:: planning
description:: Current state of a task or project
```

### `#view-definition`

Defines a database view for querying and displaying nodes using SQL syntax.

**Purpose:** Creates saved queries with display configuration.

**Built-in Properties:**

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `name` | string | Yes | View identifier |
| `query` | string | Yes | SQL query to execute |
| `display-properties` | list | No | Properties to show in view |
| `sort-by` | string | No | Property to sort by |
| `sort-order` | enum | No | asc or desc |
| `group-by` | string | No | Property to group by |

**Example:**

```bash
flow create "Active Projects View #view-definition 
  name::active-projects 
  query::\"SELECT * FROM nodes WHERE 'project' IN tags AND status = 'active'\" 
  display-properties::[name,owner,due-date,priority] 
  sort-by::priority 
  sort-order::desc"
```

---

## Built-in Primitive Properties

These properties are available on all nodes without definition.

### Core Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Node UUID (read-only) |
| `created` | date | Creation timestamp (read-only) |
| `modified` | date | Last modification timestamp (read-only) |
| `author` | string | Node creator (read-only) |

### System Properties

These properties have special behavior:

| Property | Type | Description |
|----------|------|-------------|
| `archived` | boolean | Hide from default views |
| `pinned` | boolean | Keep at top of lists |
| `favorite` | boolean | Mark as favorite |

---

## Property Type System

### Supported Types

**string**
- Any text value
- Optional `pattern` for regex validation
- Optional `min`/`max` for length constraints

**number**
- Integer or floating point
- Optional `min`/`max` for range constraints

**boolean**
- `true` or `false`

**date**
- ISO 8601 date or datetime
- Can use date references (@today, etc.)

**reference**
- NodeId pointing to another node
- Optional `reference-tag` to constrain target type
- Syntax: `@node-id` or `[[node-id]]`

**list**
- Array of values of any type
- Syntax: `[item1, item2, item3]`

**enum**
- String constrained to specific values
- Must define `values` property in definition
- Syntax: one of the allowed values

**map** (future)
- Key-value pairs
- Syntax: `{key1: value1, key2: value2}`

### Type Coercion

Automatic type coercion during property setting:

```
"5" → 5 (if property type is number)
"true" → true (if property type is boolean)
"2024-11-24" → Date (if property type is date)
```

---

## Schema Validation

When a node has a tag with a defined schema, the system validates:

### 1. Required Properties

If tag definition has `required-properties`, node must have those properties:

```rust
fn validate_required_properties(node: &Node, tag_def: &Node) -> Vec<ValidationError>
```

### 2. Property Types

All properties must match their defined types:

```rust
fn validate_property_types(node: &Node) -> Vec<ValidationError>
```

### 3. Property Constraints

Validate against min/max/pattern/values constraints:

```rust
fn validate_property_constraints(node: &Node) -> Vec<ValidationError>
```

### 4. Reference Validity

All reference properties must point to existing nodes:

```rust
fn validate_reference_properties(node: &Node, graph: &Graph) -> Vec<ValidationError>
```

---

## Building Custom Systems

### Example: Task Management System

**Step 1: Define Task Tag**

```bash
flow create "Task Tag #tag-definition 
  name::task 
  icon::✓ 
  color::#10b981 
  schema::[status,priority,assignee,due-date,estimate] 
  required-properties::[status]"
```

**Step 2: Define Properties**

```bash
# Status property
flow create "Task Status #property-definition 
  name::status 
  type::enum 
  values::[todo,in-progress,blocked,done] 
  default::todo"

# Priority property
flow create "Priority #property-definition 
  name::priority 
  type::number 
  min::1 
  max::5 
  default::3"

# Assignee property
flow create "Assignee #property-definition 
  name::assignee 
  type::reference 
  reference-tag::person"

# Due date property
flow create "Due Date #property-definition 
  name::due-date 
  type::date"

# Estimate property
flow create "Estimate #property-definition 
  name::estimate 
  type::number 
  description::\"Estimated hours\""
```

**Step 3: Define Person Tag**

```bash
flow create "Person Tag #tag-definition 
  name::person 
  icon::👤 
  color::#6366f1 
  schema::[email,role,team]"
```

**Step 4: Create Views**

```bash
# My tasks view
flow create "My Tasks #view-definition 
  name::my-tasks 
  query::\"SELECT * FROM nodes WHERE 'task' IN tags AND assignee = '@me' AND status != 'done'\" 
  display-properties::[status,priority,due-date] 
  sort-by::priority 
  sort-order::desc"

# Overdue tasks view
flow create "Overdue #view-definition 
  name::overdue-tasks 
  query::\"SELECT * FROM nodes WHERE 'task' IN tags AND due_date < CURRENT_DATE AND status != 'done'\" 
  display-properties::[assignee,due-date,priority] 
  sort-by::due-date"
```

**Step 5: Use the System**

```bash
# Create a person
flow create "Alice Smith #person email::alice@example.com role::engineer team::backend"

# Create a task
flow create "Implement query engine #task 
  status::in-progress 
  priority::5 
  assignee::@<alice-node-id> 
  due-date::2024-12-01 
  estimate::8"

# The #task tag automatically adds its template structure as children
```

### Example: Knowledge Base System

**Define Article Tag with Template**

```bash
# Create the tag definition
flow create "Article Tag #tag-definition 
  name::article 
  icon::📄 
  schema::[category,published,author,reviewed] 
  required-properties::[category,author]"

# Add template structure
flow append <article-tag-def-id> "## Overview"
flow append <article-tag-def-id> "## Content"
flow append <article-tag-def-id> "## References"
```

**Define Category Property**

```bash
flow create "Category #property-definition 
  name::category 
  type::enum 
  values::[tutorial,reference,guide,concept,api-docs]"
```

**Create View**

```bash
flow create "Published Articles #view-definition 
  name::published-articles 
  query::\"SELECT * FROM nodes WHERE 'article' IN tags AND published = true\" 
  display-properties::[category,author,reviewed] 
  sort-by::category"
```

**Use the System**

```bash
# Create article with template auto-applied
flow create "Getting Started with Flow #article 
  category::tutorial 
  author::michael 
  published::true"

# Template children automatically added:
# - ## Overview
# - ## Content  
# - ## References
```

---

## Introspection

Users can query the type system itself using SQL:

```bash
# List all tag definitions
flow query "SELECT * FROM nodes WHERE 'tag-definition' IN tags"

# Find all properties that reference specific tags
flow query "SELECT * FROM nodes WHERE 'property-definition' IN tags AND reference_tag = 'person'"

# Show schema for a tag
flow show <tag-def-node-id>

# Find all nodes using a specific tag
flow query "SELECT * FROM nodes WHERE 'project' IN tags"
```

---

## Core API for Meta-Model

### Tag Definition Management

```rust
pub fn create_tag_definition(
    graph: &mut Graph,
    name: &str,
    schema: Vec<String>,
    metadata: TagMetadata,
) -> Result<NodeId, GraphError>

pub fn get_tag_definition(
    graph: &Graph,
    tag_name: &str,
) -> Option<NodeId>

pub fn list_tag_definitions(graph: &Graph) -> Vec<NodeId>

pub fn get_tag_schema(
    graph: &Graph,
    tag_name: &str,
) -> Option<Vec<String>>
```

### Property Definition Management

```rust
pub fn create_property_definition(
    graph: &mut Graph,
    name: &str,
    property_type: PropertyType,
    constraints: PropertyConstraints,
) -> Result<NodeId, GraphError>

pub fn get_property_definition(
    graph: &Graph,
    property_name: &str,
) -> Option<NodeId>

pub fn list_property_definitions(graph: &Graph) -> Vec<NodeId>

pub fn validate_property_value(
    graph: &Graph,
    property_name: &str,
    value: &PropertyValue,
) -> Result<(), ValidationError>
```

### Schema Validation

```rust
pub fn validate_node_schema(
    graph: &Graph,
    node_id: NodeId,
) -> Vec<ValidationError>

pub fn get_validation_errors_for_tag(
    graph: &Graph,
    node_id: NodeId,
    tag_name: &str,
) -> Vec<ValidationError>

pub fn enforce_schema(
    graph: &mut Graph,
    node_id: NodeId,
) -> Result<(), GraphError>
```

### View Management

```rust
pub fn create_view(
    graph: &mut Graph,
    name: &str,
    query: &str,
    config: ViewConfig,
) -> Result<NodeId, GraphError>

pub fn execute_view(
    graph: &Graph,
    view_id: NodeId,
) -> Result<Vec<NodeId>, GraphError>

pub fn list_views(graph: &Graph) -> Vec<NodeId>
```

### Template Management

```rust
pub fn get_tag_template(
    graph: &Graph,
    tag_name: &str,
) -> Option<Vec<NodeId>>

pub fn apply_tag_with_template(
    graph: &mut Graph,
    node_id: NodeId,
    tag_name: &str,
) -> Result<(), GraphError>

pub fn has_template(
    graph: &Graph,
    tag_name: &str,
) -> bool
```

When applying a tag with `apply_tag_with_template`, the system:
1. Looks up the tag definition node
2. Gets all child nodes (the template)
3. Deep copies the template structure
4. Adds copies as children of the target node

---

## Bootstrap Process

When initializing a new graph, create the bootstrap definitions:

```rust
pub fn bootstrap_meta_model(graph: &mut Graph) -> Result<(), GraphError> {
    // Create #tag-definition itself
    let tag_def_id = create_tag_definition(
        graph,
        "tag-definition",
        vec!["name", "description", "color", "icon", "schema", "required-properties"],
        TagMetadata::default(),
    )?;
    
    // Tag it with itself (meta!)
    apply_tag(graph, tag_def_id, "tag-definition")?;
    
    // Create #property-definition
    let prop_def_id = create_tag_definition(
        graph,
        "property-definition",
        vec!["name", "type", "description", "required", "default", "values", "min", "max", "pattern"],
        TagMetadata::default(),
    )?;
    apply_tag(graph, prop_def_id, "tag-definition")?;
    
    // Create #view-definition
    let view_def_id = create_tag_definition(
        graph,
        "view-definition",
        vec!["name", "query", "display-properties", "sort-by", "sort-order", "group-by"],
        TagMetadata::default(),
    )?;
    apply_tag(graph, view_def_id, "tag-definition")?;
    
    Ok(())
}
```

This creates a self-describing system where the meta-model is defined using itself.

---

## Additional Built-in Tags (Optional)

These could be added for common use cases:

### `#relation-definition`

Define custom relationship types between nodes:

```bash
flow create "Depends On Relation #relation-definition 
  name::depends-on 
  source-tag::task 
  target-tag::task 
  inverse::blocked-by"
```

### `#automation-definition`

Define automated behaviors:

```bash
flow create "Auto-Archive Completed #automation-definition 
  name::archive-completed 
  trigger::\"prop:status=done\" 
  action::\"set prop:archived=true\""
```

### `#webhook-definition`

Define external integrations:

```bash
flow create "Slack Notification #webhook-definition 
  name::slack-notify-tasks 
  url::\"https://hooks.slack.com/...\" 
  trigger::\"tag:task AND prop:priority=5\" 
  template::\"{content} is high priority\""
```

---

## Property Inheritance

Tags can inherit schemas from other tags:

```bash
flow create "Task Tag #tag-definition 
  name::task 
  schema::[status,priority]"

flow create "Bug Tag #tag-definition 
  name::bug 
  inherits::task 
  schema::[severity,reproducible] 
  required-properties::[severity]"
```

A node with `#bug` automatically gets the schema from `#task` plus its own properties.

---

## Validation Modes

### Strict Mode

All nodes must pass validation. Invalid nodes prevented from being saved.

```bash
flow config set validation_mode strict
```

### Warn Mode

Validation errors shown as warnings but don't block operations.

```bash
flow config set validation_mode warn
```

### Permissive Mode

No validation enforcement (default for backward compatibility).

```bash
flow config set validation_mode permissive
```

---

## UI Integration

Frontends should leverage meta-model for enhanced UX:

**Tag Autocomplete:**
- Show only valid tags based on context
- Display tag icons and colors
- Show schema properties when tag selected
- Indicate if tag has template

**Property Autocomplete:**
- Show property definitions for current tags
- Display type information
- Validate as user types

**Schema Hints:**
- Show required properties for applied tags
- Highlight validation errors inline
- Suggest properties from schema

**View Rendering:**
- Execute SQL view queries
- Display configured properties
- Apply sorting and grouping

**Template Indication:**
- Show preview of template structure when selecting tag
- Indicate that child nodes will be created
- Allow opting out of template application

---

## Migration and Evolution

Schema definitions are nodes, so they're versioned and can evolve:

```bash
# Update a tag definition
flow edit <tag-def-node-id>
# Add new property to schema
schema:: [status, priority, owner, due-date, labels]  # Added 'labels'
```

Changes propagate immediately. Existing nodes don't need migration (permissive mode). Strict mode catches schema violations.

---

## Summary

Meta-model provides:

1. **Self-hosting type system** - Types defined as nodes
2. **Custom schemas** - Users define tags with properties
3. **Validation** - Type checking and constraint enforcement
4. **Views** - Saved SQL queries with display config
5. **Templates** - Tag definitions include child node templates
6. **Introspection** - Query the type system itself with SQL
7. **Evolution** - Schema changes without migration

Built-in primitives:
- `#tag-definition` - Define object types with templates
- `#property-definition` - Define typed properties
- `#view-definition` - Define SQL database views

Users build custom systems (tasks, CRM, knowledge base, etc.) on these primitives, creating Tana-like flexibility with full local control.
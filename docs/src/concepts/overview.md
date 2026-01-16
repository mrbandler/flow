# Core Concepts

This chapter explains the fundamental concepts behind Flow. Understanding these will help you get the most out of the tool.

## Outliner-First

Flow is built around the idea that **thinking happens in outlines**. Instead of starting with a blank page, you start with bullet points that can be nested, rearranged, and expanded.

```markdown
- Project ideas
  - Flow improvements
    - Add vim keybindings
    - Implement sync server
  - Blog posts to write
    - Why outliners beat documents
```

Each bullet point is a discrete unit of thought that can:

- Be collapsed or expanded
- Be referenced from elsewhere
- Have properties attached
- Be queried and filtered

## Object-Based Notes

Every item in Flow is an **object** with:

1. **Content** - The text you write
2. **References** - Links to other objects (`[[like this]]`)
3. **Tags** - Classification markers (`#tag`)
4. **Properties** - Key-value metadata (`status:: active`)

This means your notes aren't just text—they're queryable data.

## Knowledge Graph

All your notes form a **graph** of interconnected ideas:

- **Nodes** are your notes/bullets
- **Edges** are references between them
- **Tags** group related nodes
- **Properties** add structured data

You can traverse this graph, find connections, and query it like a database.

## Spaces

A **space** is a collection of notes, typically stored in a single directory. You might have:

- A personal space for life management
- A work space for project notes
- A space per major project

Spaces are independent but can reference each other.

## Local-First

Your data lives on **your machine** in plain Markdown files:

- No cloud dependency
- No vendor lock-in
- Version control friendly
- Readable without Flow

Sync is opt-in and self-hosted.

---

Continue to the next chapters to learn about each concept in detail.

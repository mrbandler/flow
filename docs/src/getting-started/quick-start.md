# Quick Start

This guide will help you get started with Flow in just a few minutes.

## Your First Capture

The most basic Flow command is `capture` - it lets you quickly save a thought without interrupting your workflow:

```bash
flow capture "Remember to review the API design"
```

That's it! Your thought is saved. You can continue with what you were doing.

## Adding Context with Tags

Make your notes more organized by adding tags:

```bash
flow capture "Implement caching layer #backend #performance"
```

Tags help you find related notes later.

## Linking Ideas

Reference other notes using double brackets:

```bash
flow capture "This relates to [[API Design]] discussion"
```

Flow automatically creates bidirectional links between your notes.

## Viewing Your Notes

List your recent captures:

```bash
flow list
```

Search across all your notes:

```bash
flow search "API"
```

## Creating a Space

Spaces help you organize different areas of your life or work:

```bash
# Initialize a new space in the current directory
flow init

# Or create a named space
flow init --name "work-notes"
```

## Next Steps

- Learn about [Core Concepts](../concepts/overview.md) to understand Flow's philosophy
- Explore the [CLI Reference](../reference/cli.md) for all available commands
- Read about [Spaces](../concepts/overview.md#spaces) to organize your knowledge

---

> **Tip:** Add `flow capture` to a shell alias for even faster capture:
>
> ```bash
> alias fc="flow capture"
> ```

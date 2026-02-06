---
allowed-tools: Bash(git diff:*), Bash(git log:*), Bash(git status:*), Read, Edit, Write, Glob, Grep, AskUserQuestion
description: Update the mdBook documentation (docs/) based on recent code changes. Fills in stub pages, updates existing pages, and adds new sections when applicable.
---

## Context

- Recent commits: !`git log --oneline -20`
- Changed files since last docs update: !`git diff HEAD --name-only`
- Current docs structure (SUMMARY.md): !`cat docs/src/SUMMARY.md 2>/dev/null || echo "No SUMMARY.md found"`

## Your task

Analyze recent code changes and update the mdBook documentation under `docs/src/` to keep it in sync with the codebase. This is a broad task — use judgment about what documentation changes are warranted.

### 1. Gather context

First, understand what changed in the code:

1. Review the git log and diff above to identify what areas of the codebase were modified
2. Read the changed source files to understand the new/modified functionality
3. Read `docs/src/SUMMARY.md` to understand the documentation structure

### 2. Determine which docs need updating

Map code changes to documentation pages. Use this mapping as a guide:

| Code area | Docs pages |
|-----------|-----------|
| `crates/flow-cli/src/commands/` | `reference/cli.md`, `guide/cli.md` |
| `crates/flow-cli/src/commands/space/` | `reference/cli.md` (space subcommands), `concepts/spaces.md` |
| `crates/flow-core/src/config/` | `reference/configuration.md`, `getting-started/configuration.md` |
| `crates/flow-core/src/space/` | `concepts/spaces.md` |
| `crates/flow-core/src/` (general) | `concepts/overview.md`, `development/architecture.md` |
| `crates/flow-common/` | `development/architecture.md` |
| `crates/flow-errors/` | `development/architecture.md` |
| New crate or module | `development/architecture.md`, possibly `SUMMARY.md` |
| `Cargo.toml` features | `getting-started/installation.md` |
| `CONTRIBUTING.md` | `development/contributing.md` |

For each identified docs page, read it to check if it is:

- **A stub** (only contains a heading, 1-3 lines) — fill it in if the code changes are directly relevant to that page's topic
- **Complete but outdated** — update the specific sections that no longer match the code
- **Complete and current** — skip it
- **Missing entirely** — if changes warrant a new page, create it and add it to `SUMMARY.md`

### 3. Writing style

Match the existing documentation style:

- **Tone**: Professional, approachable, developer-focused
- **Voice**: Direct second-person ("you")
- **Format**: Use code blocks, tables, and bullet lists liberally
- **Examples**: Concrete and realistic, not toy examples
- **Code blocks**: Use `bash` for shell commands, `rust` for Rust code, `toml` for config
- **Warnings**: Use `> **Note:**` blockquotes for caveats and early-development warnings
- **Structure**: Progressive complexity within each page (simple first, advanced later)
- **Depth**: Tutorial-level for Getting Started/Guide, reference-level for Reference section

Follow this page structure template:

```markdown
# Page Title

Brief introduction explaining what this page covers.

> **Note:** Any caveats or early development warnings.

## Section

Content with explanation.

### Subsection (if needed)

More detail.

**Arguments / Options / Fields:**

| Name | Description |
|------|-------------|
| `name` | What it does |

**Examples:**

\```bash
# Comment explaining the example
flow command --option value
\```
```

### 4. What NOT to do

- Do NOT document features that don't exist yet (aspirational content). Only document what the code actually implements.
- Do NOT remove stub pages — leave them as stubs if changes aren't relevant to them.
- Do NOT modify `docs/book.toml` or compiled output under `docs/book/`.
- Do NOT add pages to `SUMMARY.md` unless you're also creating the page content.
- Do NOT rewrite entire pages when only a section needs updating.

### 5. Execute

For each documentation change:

1. Read the target docs page
2. Read the relevant source code if needed for accuracy
3. Use Edit to update existing pages, Write only for new pages or filling stubs
4. Verify cross-references (links to other pages) are correct

Report what was updated when done, with a brief summary of changes per file.

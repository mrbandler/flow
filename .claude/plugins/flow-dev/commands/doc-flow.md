---
allowed-tools: Bash(git diff:*), Bash(git status:*), Read, Edit
description: Analyze unstaged/uncommitted Rust changes and add or update rustdoc comments following industry standard conventions. Updates stale docs that no longer match the code.
---

## Context

- Current git diff (unstaged + staged): !`git diff HEAD`
- Current git status: !`git status`

## Your task

Analyze all changed Rust files shown above and add or update rustdoc documentation comments. Only touch files that have changes — do not document unchanged files.

### What to document

For each changed file, identify all **public items** (`pub`) that are new or modified:

- Functions and methods (`pub fn`, `pub async fn`)
- Structs and their public fields
- Enums and their variants
- Traits and their methods (including default implementations)
- Type aliases
- Constants and statics
- Module-level docs (`//!`) if a new module was added

Also scan for **existing doc comments on changed items** that may now be stale (parameters renamed, return types changed, behavior altered, error conditions added/removed).

### Rustdoc style conventions

Follow the official Rust documentation standards:

**Structure for each item (in order):**

1. **Summary line** — a single concise sentence describing what the item does. This appears in module overviews and search results. Use third-person present tense ("Returns...", "Creates...", "Represents..."). No period needed if it's a sentence fragment, period if it's a full sentence.

2. **Extended description** — additional context, separated by a blank line from the summary. Explain *why* this exists, not just *what* it does. Mention design decisions or constraints when non-obvious.

3. **Standard sections** (include only when applicable):
   - `# Arguments` — for functions with non-obvious parameters. Skip if the function signature is self-explanatory.
   - `# Returns` — only when the return value needs explanation beyond the type signature.
   - `# Errors` — **required** for any function returning `Result`. List each error condition as a bullet point.
   - `# Panics` — **required** if the function can panic. Document every panic condition.
   - `# Safety` — **required** for `unsafe` functions. Document all invariants the caller must uphold.
   - `# Examples` — include for non-trivial public API items. Use `no_run` for code that requires runtime context (async, filesystem, network). Use `ignore` for illustrative snippets that won't compile standalone.
   - `# Design Notes` — for trait/architecture decisions when helpful to maintainers.

**Formatting rules:**

- Use `///` for item docs, `//!` for module/crate docs
- Cross-reference related items with `` [`ItemName`] `` or `` [`ItemName`](path::to::Item) `` syntax
- Add `#[must_use]` attributes on pure functions that return important values (but only suggest, don't add without checking)
- Document enum variants individually — at minimum a one-line summary each
- Document public struct fields individually
- Do **not** repeat type information that's already visible in the signature
- Use bullet lists for error conditions and argument descriptions
- Keep examples realistic — use the actual types from the crate

**Example patterns for this project:**

```rust
/// Initialize a new space at the given path.
///
/// Creates the directory structure and configuration files for a
/// new Flow space. The space is registered with the given name
/// for future lookup via [`Locator::Name`].
///
/// # Errors
///
/// Returns an error if:
/// - A space already exists at the given path
/// - A space with the given name is already registered
/// - The directory cannot be created
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use flow_core::Space;
///
/// # async fn example() -> miette::Result<()> {
/// let space = Space::init(Path::new("./my-notes"), "personal").await?;
/// # Ok(())
/// # }
/// ```
```

### Process

1. Read each changed file using the Read tool
2. Identify new or modified public items that need docs
3. Identify existing docs on changed items that are now stale or inaccurate
4. Use the Edit tool to add or update doc comments
5. Do NOT modify any code logic — only add/update doc comments and `//!` module docs
6. Do NOT add docs to private items unless they implement a public trait

Report what was documented when done.

---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git diff:*), Bash(git commit:*), Bash(git log:*), Bash(git cliff:*), Read, Edit, AskUserQuestion
description: Smart git commit with atomic commits and automatic CHANGELOG updates. No co-author attribution.
---

## Context

- Current git status: !`git status`
- Current git diff (staged and unstaged changes): !`git diff HEAD`
- Current branch: !`git branch --show-current`
- Recent commits: !`git log --oneline -10`

## Your task

Analyze the changes above and create git commits following these rules:

### 1. Analyze changes for atomicity

Group the changes by logical concern (feature, fix, refactor, docs, etc.). If all changes belong to a single concern, proceed with a single commit. If changes span multiple concerns, present the grouping to the user using AskUserQuestion:

- Show the proposed grouping (which files/hunks go into which commit)
- Offer options: "Split into separate commits (Recommended)", "Single commit for all changes", or let the user provide custom grouping
- If the user chooses to split, create commits one at a time in logical order (e.g., refactors before features that depend on them)

When splitting commits, use `git add <specific-files>` to stage only the files for each atomic commit. If a single file contains changes for multiple concerns, stage the whole file with the commit where it fits best and note this in the commit message.

### 2. Write commit messages

Follow Conventional Commits format: `type(scope): description`

- Types: feat, fix, docs, style, refactor, test, chore
- Scope: the affected crate or module (e.g., `cli`, `core`, `common`)
- Description: concise, lowercase, imperative mood, no period at end
- Add a body paragraph if the change is non-trivial, separated by a blank line

**CRITICAL: Do NOT add any `Co-Authored-By` line or any other trailer attributing Claude/AI as author. Never include co-author attributions of any kind.**

Pass commit messages via HEREDOC:

```
git commit -m "$(cat <<'EOF'
type(scope): description

Optional body explaining the why.
EOF
)"
```

### 3. Update CHANGELOG if applicable

After creating the commit(s), run `git cliff --unreleased --strip header` to see what git-cliff would generate for the unreleased changes. This output reflects the project's `cliff.toml` conventions (grouping, formatting, scopes).

Use the git-cliff output **as a reference alongside your own analysis** to decide what belongs in the CHANGELOG. Do NOT paste the cliff output verbatim. Instead, curate entries by applying these filters:

Changes that warrant a CHANGELOG entry:

- New features (Added)
- Bug fixes (Fixed)
- Breaking changes or behavior changes (Changed)
- Deprecations (Deprecated)
- Removals (Removed)
- Security fixes (Security)

Changes that do NOT need a CHANGELOG entry:

- Internal refactors with no behavior change
- Test-only changes
- CI/build configuration changes
- Code style/formatting changes
- Documentation-only changes (unless it's a new user-facing doc)

If a CHANGELOG entry is warranted:

1. Read the current `CHANGELOG.md` using the Read tool
2. Use the cliff output to guide the formatting and grouping style (e.g., `**scope:** description` pattern, section names)
3. Add curated entries under the `## [Unreleased]` section in the appropriate subsection
4. Use the Edit tool to add the entries
5. Stage `CHANGELOG.md` and create a separate `docs: Update CHANGELOG` commit

### 4. Execution

After determining the commit strategy (single or split), execute the commits. For each commit:

1. Stage the relevant files with `git add <specific-files>`
2. Create the commit (no co-author lines)
3. Run `git status` after the final commit to verify success

Report what was committed when done.

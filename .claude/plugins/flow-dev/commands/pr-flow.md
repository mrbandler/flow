---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git diff:*), Bash(git commit:*), Bash(git log:*), Bash(git push:*), Bash(git branch:*), Bash(git rev-parse:*), Bash(gh pr create:*), Bash(gh pr view:*), Read, Edit, AskUserQuestion
description: Push current branch and create a PR. Commits uncommitted changes first using commit-flow logic. Fills in the project's PR template. No co-author attribution.
---

## Context

- Current git status: !`git status`
- Current branch: !`git branch --show-current`
- Recent commits on this branch: !`git log --oneline -20`

## Your task

Create a pull request for the current branch using `gh` CLI. Follow these steps in order:

### 1. Handle uncommitted changes

If there are uncommitted changes (staged, unstaged, or untracked non-ignored files):

1. Analyze changes for atomicity and group by logical concern
2. If multiple concerns exist, ask the user how to split using AskUserQuestion (options: split into separate commits, single commit, custom grouping)
3. Write Conventional Commits messages: `type(scope): description`
4. Update `CHANGELOG.md` under `## [Unreleased]` if changes are user-facing (new features, bug fixes, behavior changes, deprecations, removals, security fixes). Skip for internal refactors, test-only, CI/build, formatting, or docs-only changes.
5. Stage specific files and create commits
6. **CRITICAL: No `Co-Authored-By` lines or AI attribution in any commits**

If there are no uncommitted changes, skip to step 2.

### 2. Determine the base branch and gather the diff

Identify the base branch (usually `main`). Then gather:

- All commits since diverging: `git log main..HEAD --oneline`
- Full diff: `git diff main...HEAD`

Use these to understand the full scope of changes for the PR description.

### 3. Push the branch

```
git push -u origin <current-branch>
```

Skip if already pushed and up to date.

### 4. Read the PR template

Read the file `.github/PULL_REQUEST_TEMPLATE.md` using the Read tool. This is the structure to follow for the PR body.

### 5. Fill in the PR template and create the PR

Analyze ALL commits in the branch (not just the latest) and the full diff to fill in every section of the PR template:

- **Description**: Concise summary of what this PR does and why
- **Related Issues**: Reference issues mentioned in commit messages (Fixes #N, Relates to #N), leave empty otherwise
- **Type of Change**: Check the correct box(es) based on the nature of the commits
- **Changes Made**: Bullet points derived from commit messages and the diff
- **Testing**: Check applicable boxes, add manual testing notes if relevant
- **Checklist**: Check items that were actually done

Create the PR:

```
gh pr create --title "<short title, under 70 chars>" --body "$(cat <<'EOF'
<filled-in PR template content>
EOF
)"
```

The title should use Conventional Commits style if the PR is a single concern, or a brief summary if it spans multiple.

### 6. Report

Output the PR URL when done.

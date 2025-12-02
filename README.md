<div align="center">

# Flow

**Thoughts captured. Focus unbroken.**

[![License: AGPL-3](https://img.shields.io/badge/License-AGPL--3-blue.svg)](./LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Project Status: Early Development](https://img.shields.io/badge/Status-Early%20Development-yellow.svg)]()

*An outliner with object-based note-taking that stays out of your way*

</div>

---

## ⚠️ Project Status

Flow is in **early development**. The core architecture is being built, and features are being actively developed. While the vision is clear, not all features listed below are implemented yet. Star and watch this repository to follow the progress!

---

You're deep in your work. An idea surfaces.

Most note apps make you stop what you're doing: open the app, find the right place, format it properly. By the time you're done, your train of thought is derailed.

**Flow is different.**

```bash
flow add "Research CRDT sync options for offline-first" --tag idea

# Done. Back to work.
```

Your thought is captured. Your momentum continues. Find it later when you need it.

---

## Philosophy

Flow is built on principles that put you in control:

1. **Local-First** - Your data belongs to you, stored on your machine
2. **Plain Text** - Markdown files you can read in any editor
3. **No Lock-In** - Your notes are just files, take them anywhere
4. **Outliner-First** - Organize thoughts hierarchically, add prose when needed
5. **Developer-Friendly** - Built for terminal workflows and automation
6. **Progressive Enhancement** - Start simple, add complexity as needed

**Your notes should adapt to your workflow, not the other way around.**

## Use Cases

- **Quick Capture** - Jot down ideas without leaving your terminal
- **Structured Thinking** - Organize thoughts in outlines, expand to prose when needed
- **Knowledge Base** - Build a personal wiki with automatic linking
- **Second Brain** - Build your personal knowledge management system
- **LifeOS** - Organize your entire life in one interconnected system
- **Project Notes** - Keep project-specific notes in flow directories
- **Team Documentation** - Share knowledge with self-hosted sync
- **Zettelkasten** - Build a personal knowledge graph
- **Developer Journal** - Log daily learnings and solutions

## Features

### 🎯 Core Approach

Flow is an **outliner-first** note-taking app with **object-based note-taking**. Structure your thoughts as nested bullet points, the way developers naturally think. Each note is an object that can be referenced, linked, and queried across your knowledge base. Need to write longer prose? Flow supports that too, but the default is fast, hierarchical capture with rich interconnections.

```markdown
- Project X #project <!-- n:3ads12 -->
  status:: 🟩 Active
  - Architecture decisions
    collapsed:: true
    - Use microservices for scalability
    - [[PostgreSQL]] for main DB
  - Tasks
    - [ ] Set up CI/CD pipeline #task
    - [ ] Write API documentation #task
```

### 🚀 Components

Flow is a modular system. Use what you need, when you need it:

#### 📦 **CLI** - Quick Capture
```bash
# Capture a quick thought
flow capture "Meeting notes"

# Search your knowledge base
flow search "project ideas"

# Query your knowledge graph
flow query "notes linked to #rust"
```

#### 🖥️ **TUI** - Full-Featured Terminal Interface
Browse, edit, and navigate your knowledge graph without leaving the terminal.

#### 🪟 **Desktop Application** - Rich Editing Experience
Native desktop app for longer writing sessions and onboarding less technical users.

#### 🌐 **Web + Sync Server** - Access Anywhere
Self-host a sync server (just a headless Flow instance with an API) to keep everything synchronized across devices.

#### 🔌 **Automation-Ready**
- RESTful API for integrations
- Shell script friendly
- Query language for complex data retrieval
- Integrate with n8n, Zapier, and custom workflows

### Key Principles

- **Stay in Flow** - Capture thoughts without context switching
- **Your Data, Your Rules** - Plain markdown files stored locally by default
- **Opt-in Complexity** - Start simple, add features as you need them
- **Maximum Flexibility** - Use it your way, integrate with your tools

## Roadmap

### Phase 1: Foundation (Current)
- [x] Project structure and core architecture
- [ ] Basic CLI with capture and list commands
- [ ] Local markdown storage with outliner support
- [ ] Simple search functionality

### Phase 2: Enhanced Interaction
- [ ] TUI with full navigation and editing
- [ ] Knowledge graph visualization
- [ ] Query language implementation
- [ ] Tagging and linking system

### Phase 3: Sync & Collaborate
- [ ] Sync server implementation
- [ ] Multi-device synchronization
- [ ] Conflict resolution
- [ ] Web frontend

### Phase 4: Advanced Features
- [ ] Desktop application
- [ ] Extension system
- [ ] API for automation
- [ ] Advanced query capabilities

## Getting Started

> **Note:** Flow is in early development. Installation instructions will be available soon.

## Inspiration

Flow draws inspiration from tools like:

- **Logseq** - Outliner approach and daily notes
- **Tana** - Object-based note-taking and supertags
- **Obsidian** - Knowledge graph and local-first approach
- **Roam Research** - Bidirectional linking
- **jrnl** - CLI simplicity
- **Notion** - Flexibility and blocks

I am building what I felt was missing: a developer-first, outliner-first note-taking tool that doesn't interrupt your **Flow**.

## License

Flow is licensed under the [AGPL-3 License](./LICENSE).

---

<div align="center">

**[⭐ Star this repository](https://github.com/mrbandler/flow)** if you're excited about Flow!

Built with ❤️ and Rust

</div>

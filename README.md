<div align="center">

# Flow

**Thoughts captured. Focus unbroken.**

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: AGPL-3](https://img.shields.io/badge/License-AGPL--3-blue.svg)](./LICENSE)
[![CI](https://github.com/mrbandler/flow/actions/workflows/ci.yml/badge.svg)](https://github.com/mrbandler/flow/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/mrbandler/flow/branch/master/graph/badge.svg)](https://codecov.io/gh/mrbandler/flow)

*A developer-first, outliner-based note-taking tool that stays out of your way*

[Documentation](https://mrbandler.github.io/flow/) · [Roadmap](#roadmap) · [Contributing](./CONTRIBUTING.md)

</div>

---

You're deep in your work. An idea surfaces.

Most note apps make you stop what you're doing: open the app, find the right place, format it properly. By the time you're done, your train of thought is derailed.

**Flow is different.**

```bash
flow capture "Research CRDT sync options for offline-first #idea"

# Done. Back to work.
```

Your thought is captured. Your momentum continues. Find it later when you need it.

---

## Why Flow?

- **Local-First** — Your data stays on your machine, in plain Markdown
- **Outliner-First** — Think in bullets, expand to prose when needed
- **Developer-Friendly** — CLI-native, scriptable, fits your terminal workflow
- **No Lock-In** — Your notes are just files, take them anywhere

## What You Get

| Component | Description |
|-----------|-------------|
| **CLI** | Quick capture and search from your terminal |
| **TUI** | Full-featured terminal interface |
| **GUI** | Native desktop app for longer sessions |
| **Server** | Self-hosted sync across devices |

Mix and match — use only what you need.

## Quick Example

```markdown
- Project X #project
  status:: 🟩 Active
  - Architecture decisions
    - Use microservices for scalability
    - [[PostgreSQL]] for main DB
  - Tasks
    - [ ] Set up CI/CD pipeline #task
    - [ ] Write API documentation #task
```

Every bullet is a node. Add tags to create objects you can query and connect across your knowledge graph.

## Roadmap

> ⚠️ **Flow is in early development.** Star and watch to follow progress!

- [x] Project structure and architecture
- [ ] Core storage layer (Markdown + Loro CRDT)
- [ ] CLI integration
- [ ] TUI interface
- [ ] GUI application
- [ ] Sync server

## Getting Started

```bash
# Coming soon — Flow is in active development
# Star the repo to get notified when it's ready!
```

📖 **[Read the documentation](https://mrbandler.github.io/flow/)** for more details.

## Contributing

Contributions are welcome! Please read the [Contributing Guide](./CONTRIBUTING.md) to get started.

## Inspiration

Flow draws inspiration from [Logseq](https://logseq.com/), [Tana](https://tana.inc/), [Obsidian](https://obsidian.md/), [Roam Research](https://roamresearch.com/), and [jrnl](https://jrnl.sh/) — combining the best of outliners, knowledge graphs, and CLI simplicity.

## License

[AGPL-3.0](./LICENSE)

---

<div align="center">

**[⭐ Star this repo](https://github.com/mrbandler/flow)** if you're excited about Flow!

</div>

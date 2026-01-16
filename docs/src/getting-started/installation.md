# Installation

This guide covers how to install Flow on your system.

## Pre-built Binaries

The easiest way to install Flow is to download a pre-built binary from the [GitHub Releases](https://github.com/mrbandler/flow/releases) page.

### Binary Variants

Flow comes in several variants depending on your needs:

| Variant | Description | Best For |
|---------|-------------|----------|
| `flow` | CLI only | Terminal users, scripts, automation |
| `flow-tui` | CLI + Terminal UI | Terminal power users |
| `flow-gui` | CLI + Desktop GUI | Users preferring graphical interfaces |
| `flow-ui` | CLI + TUI + GUI | Users who want both interfaces |
| `flow-server` | CLI + Server | Self-hosting, multi-device sync |
| `flow-fat` | Everything | Users who want all features |

### Download

1. Go to the [Releases page](https://github.com/mrbandler/flow/releases)
2. Download the appropriate archive for your platform:
   - **Linux**: `flow-<variant>-linux-x86_64.tar.gz` or `flow-<variant>-linux-aarch64.tar.gz`
   - **macOS**: `flow-<variant>-macos-x86_64.tar.gz` (Intel) or `flow-<variant>-macos-aarch64.tar.gz` (Apple Silicon)
   - **Windows**: `flow-<variant>-windows-x86_64.zip`

3. Extract and install:

#### Linux / macOS

```bash
# Extract (example for CLI variant on Linux x86_64)
tar -xzf flow-linux-x86_64.tar.gz

# Move to a directory in your PATH
sudo mv flow /usr/local/bin/

# Verify installation
flow --version
```

#### Windows

1. Extract the `.zip` file
2. Move `flow.exe` to a directory in your PATH (e.g., `C:\Users\<username>\bin`)
3. Open a new terminal and verify:

```powershell
flow --version
```

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, version 1.91.1 or later)
- Git

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/mrbandler/flow.git
cd flow

# Build the CLI (default)
cargo build --release --package flow

# Or build with specific features
cargo build --release --package flow --features tui      # CLI + TUI
cargo build --release --package flow --features gui      # CLI + GUI
cargo build --release --package flow --features all      # Everything
```

The binary will be at `target/release/flow` (or `target/release/flow.exe` on Windows).

### Install via Cargo

```bash
# Install the CLI
cargo install --git https://github.com/mrbandler/flow.git flow

# Install with TUI
cargo install --git https://github.com/mrbandler/flow.git flow --features tui
```

## Using Nix

If you use Nix, you can enter a development shell with all dependencies:

```bash
# Clone the repository
git clone https://github.com/mrbandler/flow.git
cd flow

# Enter the development shell
nix develop
```

## Verifying Your Installation

After installation, verify Flow is working:

```bash
# Check version (shows compiled features)
flow --version
# Output: flow 0.1.0 (tui, gui)  -- features vary by variant

# Show help
flow --help
```

## Next Steps

Now that Flow is installed, head to the [Quick Start](./quick-start.md) guide to begin using it!

# Flow Development Justfile

# Install all development dependencies
setup:
    rustup show
    cargo install cargo-deny --locked
    cargo install mdbook --locked
    cargo install prek --locked
    cargo fetch
    prek install

# Start Claude Code with project plugins (works around superpowers startup issue)
claude:
    -claude plugin disable superpowers@superpowers-marketplace
    claude --plugin-dir .claude/plugins/flow-dev

# Run pre-commit hooks on all files
lint:
    prek run --all-files

# Format all code
fmt:
    cargo fmt --all

# Run clippy lints
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all tests
test *ARGS:
    cargo test --workspace --all-features {{ ARGS }}

# Run dependency audit
deny:
    cargo deny check

# Build API docs
doc:
    cargo doc --workspace --all-features --no-deps

# mdBook documentation: build, dev (default: build)
book variant="build":
    @if [ "{{ variant }}" = "build" ]; then \
        mdbook build docs; \
    elif [ "{{ variant }}" = "dev" ]; then \
        mdbook serve docs --open; \
    else \
        echo "Unknown variant '{{ variant }}'. Use: build, dev"; \
        exit 1; \
    fi

# Run all checks (use before submitting a PR)
check: fmt clippy test deny doc

# Build a specific variant: cli, tui, gui, server, all (default: cli)
build variant="cli":
    @if [ "{{ variant }}" = "cli" ]; then \
        cargo build --package flow; \
    elif [ "{{ variant }}" = "tui" ]; then \
        cargo build --package flow --features tui; \
    elif [ "{{ variant }}" = "gui" ]; then \
        cargo build --package flow --features gui; \
    elif [ "{{ variant }}" = "server" ]; then \
        cargo build --package flow --features server; \
    elif [ "{{ variant }}" = "all" ]; then \
        cargo build --package flow --features all; \
    else \
        echo "Unknown variant '{{ variant }}'. Use: cli, tui, gui, server, all"; \
        exit 1; \
    fi

# Clean all test data (spaces and config)
clean:
    rm -rf ./spaces/*
    rm -rf ~/.config/flow/*

# Build and run a specific variant: cli, tui, gui
run variant *ARGS:
    @if [ "{{ variant }}" = "cli" ]; then \
        cargo run --package flow -- {{ ARGS }}; \
    elif [ "{{ variant }}" = "tui" ]; then \
        cargo run --package flow --features tui -- tui {{ ARGS }}; \
    elif [ "{{ variant }}" = "gui" ]; then \
        cargo run --package flow --features gui -- gui {{ ARGS }}; \
    else \
        echo "Unknown variant '{{ variant }}'. Use: cli, tui, gui"; \
        exit 1; \
    fi

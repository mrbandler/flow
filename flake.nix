{
  description = "Flow - Developer-first note-taking tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Read the rust-toolchain.toml to get the correct version
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Common build inputs for all platforms
        buildInputs = with pkgs; [
          # Rust toolchain
          rustToolchain

          # Build essentials
          pkg-config
          openssl

          # Development tools
          cargo-deny
          cargo-watch
          git-cliff
          mdbook

          # Pre-commit
          pre-commit
        ] ++ lib.optionals stdenv.isDarwin [
          # macOS specific dependencies
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
          darwin.apple_sdk.frameworks.CoreFoundation
        ] ++ lib.optionals stdenv.isLinux [
          # Linux specific dependencies for GUI
          # Uncomment when GUI dependencies are needed
          # gtk3
          # glib
          # cairo
          # pango
          # gdk-pixbuf
          # atk
        ];

        # Environment variables
        shellEnv = {
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          env = shellEnv;

          shellHook = ''
            echo "🚀 Flow development environment"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo clippy         - Run linter"
            echo "  cargo fmt            - Format code"
            echo "  cargo deny check     - Check dependencies"
            echo "  git-cliff            - Generate changelog"
            echo "  mdbook serve docs    - Preview documentation"
            echo "  pre-commit run -a    - Run all pre-commit hooks"
            echo ""
          '';
        };

        # Package output (for future use)
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "flow";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          meta = with pkgs.lib; {
            description = "Flow - Developer-first note-taking tool";
            homepage = "https://github.com/mrbandler/flow";
            license = licenses.agpl3Plus;
            maintainers = [ ];
          };
        };
      }
    );
}

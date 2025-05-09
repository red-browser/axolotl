{ pkgs, craneLib }:

let
  rustToolchain = pkgs.rustToolchain.override {
    extensions = [ "rust-src" "rust-analyzer" "llvm-tools-preview" ];
    targets = [ "wasm32-unknown-unknown" ];
  };
in
pkgs.mkShell {
  name = "axolotl-dev";

  RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
  CARGO_TARGET_DIR = "./target";
  HISTFILE = "./.bash_history";

  nativeBuildInputs = with pkgs; [
    # Rust
    rustToolchain
    cargo-edit
    cargo-watch
    cargo-udeps
    cargo-audit
    cargo-outdated
    rust-analyzer

    # Bazel ecosystem
    bazel_6
    bazel-buildtools
    buildifier
    buildozer

    # Nix
    nixpkgs-fmt
    rnix-lsp
    cachix

    # Debugging
    lldb
    flamegraph
    valgrind
    pkg-config
    cmake
    ninja
    clang
    gdb
    nodejs
    wasm-pack
  ];

  # Environment variables
  shellHook = ''
    export PS1="\[\e[1;32m\][axolotl:\w]\$\[\e[0m\] "
    echo "=== axolotl engine ==="
    echo "Rust: $(rustc --version)"
    echo "Bazel: $(bazel --version)"
    echo "Available commands:"
    echo "  cargo build - Build with Cargo"
    echo "  bazel build //... - Build with Bazel"
    echo "  cargo clippy - Run linter"
    echo "  cargo test - Run tests"
  '';
}

# axolotl | a trail to develop a browser engine

## Project Overview

A trailblazing experimental browser engine written in Rust, bootstrapped using Bazel and developed in a reproducible, declarative 
environment powered by Nix.

## Repository Structure

```
axolotl/
├── .bazelrc                 # Bazel configuration file
├── BUILD.bazel              # Main Bazel build file
├── CODE_OF_CONDUCT.md       # Project code of conduct
├── LICENSE.md               # Apache 2.0 license
├── README.md                # Project overview and documentation
├── WORKSPACE.bazel          # Bazel workspace configuration
├── brand/                   # Project branding assets
│   └── README.md            # Documentation for branding assets
├── flake.lock               # Nix flake lock file
├── flake.nix                # Nix flake configuration
├── nix/                     # Nix-specific configurations
│   ├── cache.nix            # Nix cache configuration
│   ├── ci.nix               # Continuous integration configuration
│   └── devShell.nix         # Development shell configuration
├── rust-toolchain.toml      # Rust toolchain configuration
├── shell.nix                # Simplified Nix shell configuration
└── src/                     # Source code directory
    └── main.rs              # Initial Rust entry point
```

## Build Systems

The project employs multiple build systems to ensure flexibility across different environments:

### Bazel Build System

The project uses Bazel as its primary build system:

- **WORKSPACE.bazel**: Configures external dependencies including:
  - Rust rules (rules_rust 0.61.0)
  - C++ rules
  - Java rules (possibly for tooling)
  - Platform specifications

- **BUILD.bazel**: Defines the main Rust binary target:
  ```bazel
  rust_binary(
      name = "axolotl",
      srcs = ["src/main.rs"],
      edition = "2021",
  )
  ```

- **.bazelrc**: Contains environment configurations for Bazel builds, including:
  - Environment variable propagation
  - Toolchain resolution settings
  - Linker configurations

### Nix Package Manager Integration

The project utilizes Nix for reproducible development environments:

- **flake.nix**: Defines the project's Nix flake with:
  - Dependencies on nixpkgs, flake-utils, and rust-overlay
  - Development shell configuration with Rust 1.70.0 and Bazel 6
  - System-specific configurations (currently x86_64-linux)

- **nix/devShell.nix**: Provides an expanded development environment with additional tools:
  - Rust development tools (cargo-edit, cargo-watch, etc.)
  - Bazel ecosystem tools
  - Debugging utilities

- **nix/ci.nix**: Configures continuous integration builds with:
  - Cargo artifact caching
  - Clippy checks
  - Test execution
  - Both Bazel and Cargo build paths

- **nix/cache.nix**: Defines Nix caching configuration for faster builds

## Rust Configuration

- **rust-toolchain.toml**: Specifies Rust toolchain requirements:
  - Stable channel
  - Components: rustc, cargo, rustfmt, clippy, rust-src, llvm-tools-preview
  - Target architecture (x86_64-unknown-linux-gnu)
  - Linker and flag configurations

- **src/main.rs**: currently a minimal "Hello" application:
  ```rust
  fn main() { println!("Hello"); }
  ```

## Contributing

All contributors must follow the [Code of Conduct](./CODE_OF_CONDUCT.md).

## License

All code under this repository is licensed under the [APACHE LICENSE](./LICENSE.md)
license.

## Getting Started

Developers can start working on the project using either

### Nix Development Environment

```bash
# Using flake
nix develop

# Using traditional shell.nix
nix-shell
```

### Bazel Build

```bash
bazel build //...
```

### Cargo Build

```bash
cargo build
```

The current focus is on establishing a solid development environment and build infrastructure before the actual browser engine implementation begins.
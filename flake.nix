{
  description = "axolotl browser engine";

 inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { 
          inherit system overlays;
          config.allowUnfree = true;
        };
        rustToolchain = pkgs.rust-bin.stable."1.80.1".default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
        basePackages = [
          rustToolchain
          pkgs.bazel_7
          pkgs.cargo-bazel
          pkgs.bazel-buildtools
          pkgs.gcc
          pkgs.glibc
          pkgs.stdenv.cc.cc.lib
          pkgs.gnumake
          pkgs.cargo-raze
          pkgs.zlib
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          name = "axolotl-dev";
          nativeBuildInputs = basePackages;
          
          buildInputs = with pkgs; [
            stdenv.cc.cc.lib
            glibc
            zlib
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.zlib
            pkgs.glibc
          ];
      
          
          shellHook = ''
          export PATH="${rustToolchain}/bin:$PATH"
          export CARGO_HOME=$(mktemp -d)
          export RUSTUP_HOME=$(mktemp -d)
          export JAVA_HOME="${pkgs.jdk}"
          export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.glibc}/lib:$LD_LIBRARY_PATH"
          export RUSTFLAGS="-C link-arg=-fuse-ld=bfd"
          echo "Using Rust: $(rustc --version)"
          echo "Using GCC: $(gcc --version | head -n1)"
          '';
        };
      }
    );
}

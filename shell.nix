{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    bazel_6
    nix
    cacert
  ];

  # Bazel needs these to find the Nix store
  BAZEL_USE_CPP_ONLY_TOOLCHAIN = "1";
  USE_BAZEL_VERSION = pkgs.bazel_6.version;
  
  # To Make sure Bazel can access the network through Nix's certificates
  SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
  GIT_SSL_CAINFO = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
}

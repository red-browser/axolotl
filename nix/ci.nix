{ pkgs, craneLib, doCheck ? true }:

let
  commonArgs = {
    src = craneLib.cleanCargoSource (craneLib.path ./.);
    nativeBuildInputs = with pkgs; [
      bazel
      pkg-config
      cmake
      ninja
    ];
    
    BAZEL_USE_CPP_ONLY_TOOLCHAIN = "1";
    USE_BAZEL_VERSION = pkgs.bazel.version;
  };

  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "axolotl-deps";
  });

  clippyCheck = craneLib.cargoClippy (commonArgs // {
    inherit cargoArtifacts;
    cargoClippyExtraArgs = "--all-targets -- --deny warnings";
  });

  # Run tests
  testRun = craneLib.cargoTest (commonArgs // {
    inherit cargoArtifacts;
  });

  # Build the final package
  finalBuild = craneLib.buildPackage (commonArgs // {
    inherit cargoArtifacts;
    doCheck = false; # Tests run separately
  });

  bazelBuild = pkgs.stdenv.mkDerivation {
    name = "axolotl-bazel-build";
    src = ./.;
    nativeBuildInputs = with pkgs; [ bazel ];
    buildPhase = ''
      bazel build --config=opt //...
    '';
    installPhase = ''
      mkdir -p $out/bin
    '';
  };
in
pkgs.stdenv.mkDerivation {
  name = "axolotl-ci";
  src = ./.;

  phases = [ "unpackPhase" "buildPhase" "checkPhase" "installPhase" ];

  buildInputs = with pkgs; [
    bazel
    rustToolchain
  ];

  buildPhase = ''
    # Build with Bazel
    ${bazelBuild.buildPhase}
    
    # Build with Cargo
    cp -r ${finalBuild} ./cargo-build
  '';

  checkPhase = pkgs.lib.optionalString doCheck ''
    # Run cargo tests
    ${testRun.checkPhase}
    
    # Run clippy
    ${clippyCheck.checkPhase}
  '';

  installPhase = ''
    mkdir -p $out/bin
  '';

  passthru = {
    inherit cargoArtifacts clippyCheck testRun finalBuild bazelBuild;
  };
}

{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , rust-overlay
    }:

    let
      cargoSha256 = "sha256-DY0V7c2Dn/Ium5vdMOga5NXQOFohHDCqIF+AQ9FsPHY=";

      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      inherit (meta) name version;

      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];
    in
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system overlays; };
      inherit (pkgs) mkShell writeScriptBin;

      xFunc = cmd: writeScriptBin "x-${cmd}" ''
        cargo watch -x ${cmd}
      '';

      ci = writeScriptBin "ci" ''
        cargo fmt --check
        cargo clippy
        cargo build --release
        cargo test
      '';

      scripts = [
        ci
        (builtins.map (cmd: xFunc cmd) [ "build" "check" "run" "test" ])
      ];
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = name;
        inherit version;
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

      devShells.default = mkShell {
        buildInputs = (with pkgs; [
          rustToolchain
          cargo-edit
          cargo-watch
          rust-analyzer
          cachix
          direnv
        ]) ++ scripts;
      };
    });
}

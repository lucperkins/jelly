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

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
      inherit (pkgs) mkShell writeScriptBin;

      xFunc = cmd: writeScriptBin "x-${cmd}" ''
        cargo watch -x ${cmd}
      '';

      ci = writeScriptBin "ci" ''
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
      devShells.default = mkShell {
        buildInputs = (with pkgs; [
          rustToolchain
          cargo-edit
          cargo-watch
          rust-analyzer
        ]) ++ scripts;
      };
    });
}

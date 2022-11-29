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

      xFunc = cmd: pkgs.writeScriptBin "x-${cmd}" ''
        cargo watch -x ${cmd}
      '';

      scripts = with pkgs; [
        (xFunc "check")
        (xFunc "run")
        (xFunc "test")

        (writeScriptBin "ci" ''
          cargo clippy
          cargo build --release
          cargo test
        '')
      ];
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = (with pkgs; [
          rustToolchain
          cargo-edit
          cargo-watch
          rust-analyzer
          zlib
        ]) ++ scripts;
      };
    });
}

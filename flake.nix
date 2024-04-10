{
  description = "Jelly: a golden path static site generator for documentation";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.*.tar.gz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*.tar.gz";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-schemas
    }:

    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            self.overlays.default
          ];
        };
      });

      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      inherit (meta) name version;
    in
    {
      inherit (flake-schemas) schemas;

      overlays.default = final: prev: {
        rustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      };

      devShells = forEachSupportedSystem ({ pkgs }: {
        default =
          let
            xFunc = cmd: pkgs.writeScriptBin "x-${cmd}" ''
              cargo watch -x ${cmd}
            '';

            ci = pkgs.writeScriptBin "ci" ''
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
          pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-edit
              cargo-watch
            ] ++ scripts;
          };
      });

      packages = forEachSupportedSystem ({ pkgs }: {
        default =
          let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
          in
          rustPlatform.buildRustPackage {
            pname = name;
            inherit version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
      });
    };
}

{
  description = "Jelly: a golden path static site generator for documentation";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.*";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*";
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
            ci = pkgs.writeShellApplication {
              name = "ci";
              runtimeInputs = with pkgs; [ rustToolchain ];
              text = ''
                cargo fmt --check
                cargo clippy
                cargo build --release
                cargo test
                nix build
              '';
            };

            dev = pkgs.writeShellApplication {
              name = "dev";
              runtimeInputs = with pkgs; [ bacon ];
              text = ''
                bacon check
              '';
            };

            scripts = [
              ci
              dev
            ];
          in
          pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-edit
              cargo-machete
              cargo-minimize
              bacon
              rust-analyzer
              static-web-server
            ]
            ++ scripts
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ CoreServices ]);
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
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin
              (with pkgs.darwin.apple_sdk.frameworks; [ CoreServices ]);
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
      });
    };
}

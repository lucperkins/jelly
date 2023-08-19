{
  description = "Jelly";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    }:

    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; overlays = self.overlays; };
      });

      meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
      inherit (meta) name version;
    in
    {
      overlays = [
        rust-overlay.overlays.default
        (self: super: {
          rustToolchain = super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        })
      ];

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
              (builtins.map (cmd: xFunc cmd) [ "build" "check" "clippy" "run" "test" ])
            ];
          in
          pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              cargo-edit
              cargo-watch
            ] ++ scripts ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ CoreServices ]);
          };

        RUST_LOG = "trace";
      });

      packages = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          inherit version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      });
    };
}

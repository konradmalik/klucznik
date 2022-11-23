{
  description = "klucznik";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
        rustPlatform = pkgs.rustPlatform;
      in
      {
        # nix build
        packages.default = rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          # run tests in check phase in debug mode instead of release
          # tests run on each build automatically
          checkType = "debug";
        };


        # nix flake check
        checks =
          {
            cargo-check = pkgs.runCommand "cargo-check"
              {
                nativeBuildInputs = with pkgs; [ cargo ];
              }
              ''
                cp -r ${./.}/. ./
                cargo check
                touch $out
              '';
            rustfmt = pkgs.runCommand "rustfmt"
              {
                nativeBuildInputs = with pkgs; [ cargo rustfmt ];
              }
              ''
                cargo fmt --manifest-path ${./.}/Cargo.toml --all --check
                touch $out
              '';
            clippy = pkgs.runCommand "clippy"
              {
                nativeBuildInputs = with pkgs; [ cargo clippy ];
              }
              ''
                cp -r ${./.}/. ./
                cargo clippy -- -D warnings
                touch $out
              '';
            nixfmt = pkgs.runCommand "nixfmt"
              {
                nativeBuildInputs = with pkgs; [ nixpkgs-fmt ];
              }
              ''
                nixpkgs-fmt --check ${./.}
                touch $out
              '';
          };

        # nix develop
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustfmt
            clippy
            nixpkgs-fmt
          ];
        };

        # nix fmt
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}

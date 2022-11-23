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
        name = cargoToml.package.name;
        commonArgs = {
          src = with pkgs.lib; cleanSourceWith {
            src = self;
            # a function that returns a bool determining if the path should be included in the cleaned source
            filter = path: type:
              let
                # filename
                baseName = builtins.baseNameOf (toString path);
                # path from root directory
                path' = builtins.replaceStrings [ "${self}/" ] [ "" ] path;
                # checks if path is in the directory
                inDirectory = directory: hasPrefix directory path';
              in
              inDirectory "src" ||
              inDirectory "tests" ||
              hasPrefix "Cargo" baseName ||
              baseName == "info.toml";
          };
          cargoLock.lockFile = ./Cargo.lock;
          version = cargoToml.package.version;
        };
      in
      {
        # nix build
        packages = {
          default = self.packages.${system}.klucznik;

          klucznik = pkgs.rustPlatform.buildRustPackage (commonArgs // {
            pname = name;
          });

          clippy = pkgs.rustPlatform.buildRustPackage (commonArgs // {
            pname = "${name}-clippy";
            doCheck = false;
            buildPhase = ''
              cargo clippy -- --deny warnings 2>&1 | tee clippy.txt
            '';
            installPhase = ''
              mkdir -p $out/reports
              cp clippy.txt $out/reports/
            '';
            nativeBuildInputs = with pkgs; [ clippy ];
          });
        };

        # nix flake check
        # checks have to access to the internet and no home
        checks =
          {
            rustfmt = pkgs.runCommand "rustfmt"
              {
                nativeBuildInputs = with pkgs; [ cargo rustfmt ];
              }
              ''
                cargo fmt --manifest-path ${./.}/Cargo.toml --all --check
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
            rustc
            rust-analyzer
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

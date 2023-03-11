{
  description = "klucznik";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };
        overlay = (final: prev: {
          klucznik = (final.callPackage self { } // {
            shell = final.callPackage ./shell.nix { };
          });
        });
      in
      {
        packages = {
          default = pkgs.klucznik;
          klucznik = pkgs.klucznik;
        };

        devShells.default = pkgs.klucznik.shell;

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}

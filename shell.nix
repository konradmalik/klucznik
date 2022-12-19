let
  flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
  nixpkgsRev = flakeLock.nodes.nixpkgs.locked.rev;
  lockedNixpkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/${flakeLock.nodes.nixpkgs.locked.rev}.tar.gz") { };
in
{ pkgs ? lockedNixpkgs }:
with pkgs;
let
  klucznik = callPackage ./default.nix { };
in
mkShell {
  nativeBuildInputs = klucznik.klucznik.buildInputs ++ [
    rustc
    cargo
    clippy
    rustfmt
    nixpkgs-fmt
    rust-analyzer
    yaml-language-server
  ];
}

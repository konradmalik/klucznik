let
  flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
  nixpkgsRev = flakeLock.nodes.nixpkgs.locked.rev;
  lockedNixpkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/${nixpkgsRev}.tar.gz") { };
in
{ pkgs ? lockedNixpkgs }:
with pkgs;
let
  klucznik = callPackage ./default.nix { };
in
mkShell {
  packages = klucznik.nativeBuildInputs ++ [
    clippy
  ];
}

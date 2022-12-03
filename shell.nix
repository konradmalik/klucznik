{ pkgs ? import <nixpkgs> { }, klucznik }:

with pkgs;
mkShell {
  nativeBuildInputs = klucznik.klucznik.buildInputs ++ [
    rustc
    cargo
    clippy
    rustfmt
    nixpkgs-fmt
    rust-analyzer
  ];
}

{ lib
, stdenv
, rustPlatform
, darwin
, rustfmt
, clippy
}:
let
  cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  cargoLock.lockFile = ./Cargo.lock;
  darwinBuildInputs = [
    darwin.apple_sdk.frameworks.Security
  ];
in
rustPlatform.buildRustPackage {
  inherit pname version cargoLock;
  src = ./.;
  nativeBuildInputs = [ clippy rustfmt ];
  buildInputs = [ ] ++ lib.optionals stdenv.isDarwin darwinBuildInputs;
  preBuildPhases = [ "cargoFmt" ];
  cargoFmt = ''
    cargo fmt --manifest-path ./Cargo.toml --all --check
  '';
  # right after checkPhase (tests)
  preInstallPhases = [ "clippy" ];
  clippy = ''
    cargo clippy -- --deny warnings
  '';
}

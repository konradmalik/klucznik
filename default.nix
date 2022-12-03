{ stdenv
, lib
, runCommand
, rustPlatform
, darwin
, nixpkgs-fmt
, rustfmt
, clippy
, cargo
}:
let
  namedSrc = builtins.path { path = ./.; name = "klucznik"; };
  src = with lib; cleanSourceWith {
    src = namedSrc;
    # a function that returns a bool determining if the path should be included in the cleaned source
    filter = path: type:
      let
        # filename
        baseName = builtins.baseNameOf (toString path);
        # path from root directory
        path' = builtins.replaceStrings [ "${namedSrc}/" ] [ "" ] path;
        # checks if path is in the directory
        inDirectory = directory: hasPrefix directory path';
      in
      inDirectory "src" ||
      inDirectory "tests" ||
      hasPrefix "Cargo" baseName;
  };
  cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  cargoLock.lockFile = ./Cargo.lock;
  darwinBuildInputs = [
    darwin.apple_sdk.frameworks.Security
  ];
in
{
  klucznik = rustPlatform.buildRustPackage {
    src = src;
    pname = pname;
    version = version;
    cargoLock = cargoLock;
    buildInputs = [ ] ++ lib.optionals stdenv.isDarwin darwinBuildInputs;
  };

  clippy = rustPlatform.buildRustPackage {
    src = src;
    pname = "${pname}-clippy";
    version = version;
    cargoLock = cargoLock;
    nativeBuildInputs = [ clippy ];
    doCheck = false;
    buildPhase = ''
      cargo clippy -- --deny warnings 2>&1 | tee clippy.txt
    '';
    installPhase = ''
      mkdir -p $out/reports
      cp clippy.txt $out/reports/
    '';
  };

  rustfmt = runCommand "rustfmt"
    {
      nativeBuildInputs = [ cargo rustfmt ];
    }
    ''
      cargo fmt --manifest-path ${src}/Cargo.toml --all --check
      touch $out
    '';

  nixfmt = runCommand "nixfmt"
    {
      nativeBuildInputs = [ nixpkgs-fmt ];
    }
    ''
      nixpkgs-fmt --check ${src}
      touch $out
    '';
}

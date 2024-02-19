{ stdenv
, rustPlatform
, cargo
, rustc
, pkg-config
, nixdoc
, iconv
, lib
}:

stdenv.mkDerivation {
  pname = "mdbook-nixdoc";
  version = "0.1";

  src = ./.;

  nativeBuildInputs = [
    rustPlatform.cargoSetupHook
    rustPlatform.cargoBuildHook
    rustPlatform.cargoInstallHook
    cargo
    rustc
    pkg-config
    nixdoc
  ]
  ++ lib.optionals stdenv.isDarwin [ iconv ];

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  cargoBuildType = "release";
  cargoCheckType = "release";

  cargoRoot = ".";
}

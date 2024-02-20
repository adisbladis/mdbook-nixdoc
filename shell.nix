{ pkgs ? import <nixpkgs> { }
, mkShell ? pkgs.mkShell
, rustc ? pkgs.rustc
, cargo ? pkgs.cargo
, rustfmt ? pkgs.rustfmt
, nixdoc ? pkgs.nixdoc
}:

mkShell {
  packages = [
    rustc
    cargo
    nixdoc
    rustfmt
  ];
}

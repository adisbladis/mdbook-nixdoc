{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";


    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, nix-github-actions, ... }@inputs:
    let
      inherit (nixpkgs) lib;

    in
    inputs.flake-parts.lib.mkFlake
      { inherit inputs; }
      {
        systems = lib.systems.flakeExposed;

        imports = [
          inputs.treefmt-nix.flakeModule
        ];

        flake.githubActions = nix-github-actions.lib.mkGithubMatrix {
          checks = { inherit (self.checks) x86_64-linux; };
        };

        perSystem = { pkgs, system, ... }:
          let
            craneLib = inputs.crane.lib.${system};
          in
          {
            treefmt.imports = [ ./dev/treefmt.nix ];

            checks = self.packages.${system};

            devShells.default = pkgs.mkShell {
              packages = [
                pkgs.rustc
                pkgs.cargo
                pkgs.nixdoc
              ];
            };

            packages.default = craneLib.buildPackage {
              src = self;
              nativeBuildInputs =
                [ pkgs.nixdoc ]
                ++ lib.optionals pkgs.stdenv.isDarwin [ pkgs.iconv ];
            };
          };
      };
}

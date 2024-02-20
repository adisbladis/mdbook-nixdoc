{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, nix-github-actions }: (
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      flake.githubActions = nix-github-actions.lib.mkGithubMatrix {
        checks = {
          inherit (self.checks) x86_64-linux;
          x86_64-darwin = {
            inherit (self.packages.x86_64-darwin) default;
          };
        };
      };

      checks =
        forAllSystems
        (
          system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            mdbook-nixdoc = self.packages.${system}.default;

            rustfmt = pkgs.runCommand "rustfmt-check" { nativeBuildInputs = [ pkgs.rustfmt]; } ''
              rustfmt --check ${self}/src/*.rs
              touch $out
            '';

          }
        );

      packages =
        forAllSystems
        (
          system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = pkgs.callPackage ./default.nix { };
          }
        );

      devShells =
        forAllSystems
        (
          system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = pkgs.callPackage ./shell.nix { };
          }
        );
    }
  );
}

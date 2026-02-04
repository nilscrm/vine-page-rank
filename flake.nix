{
  description = "Vine Page Rank";

  inputs = {
    nixpkgs.follows = "vine/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    vine.url = "github:VineLang/vine/dev";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      vine,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        vineCli = vine.packages.${system}.vine;
      in
      {
        formatter = pkgs.nixfmt-tree;

        checks.run-page-rank =
          pkgs.runCommand "run-page-rank"
            {
              nativeBuildInputs = [ vineCli ];
            }
            ''
              export HOME=$TMPDIR
              vine run --no-stats page_rank=${./page_rank.vi}
              mkdir -p $out
            '';

        devShells.default = pkgs.mkShell {
          packages = [
            vineCli
          ];
        };
      }
    );
}

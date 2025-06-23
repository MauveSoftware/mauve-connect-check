{
  description = "mauve-connect-check - A command-line tool for verifying DNS configuration of domains in the Mauve Cloud infrastructure";

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      pkgsForSystem =
        system:
        (import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        });
    in
    {
      overlays.default =
        _final: prev:
        let
          inherit (prev) rustPlatform callPackage lib;
        in
        {
          mauve-connect-check = callPackage ./package.nix { inherit rustPlatform lib; };
        };

      packages = forAllSystems (system: rec {
        inherit (pkgsForSystem system) mauve-connect-check;
        default = mauve-connect-check;
      });
    };
}

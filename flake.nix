{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/*.tar.gz";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      allSystems = [
        "x86_64-linux"
      ];

      forEachSystem =
        f:
        nixpkgs.lib.genAttrs allSystems (
          system:
          f {
            inherit system;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
              ];
            };
          }
        );

    in
    {
      devShells = forEachSystem (
        { pkgs, system }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              openssl

              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
              })
            ];
          };
        }
      );
    };
}

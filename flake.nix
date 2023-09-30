{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    pre-commit.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    pre-commit,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
      libPath = with pkgs;
        lib.makeLibraryPath [
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
    in {
      defaultPackage = naersk-lib.buildPackage ./.;

      devShell = with pkgs;
        mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;

          buildInputs = [
            cargo
            rustc
            rust-analyzer
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };

      formatter = pkgs.alejandra;
      checks.pre-commit-check = pre-commit.lib.${system}.run {
        src = ./.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
          taplo.enable = true;
        };
      };
    });
}

{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , utils
    , naersk
    ,
    }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      naersk-lib = pkgs.callPackage naersk { };
      libPath = with pkgs; lib.makeLibraryPath [
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
      ];
    in
    {
      defaultPackage = naersk-lib.buildPackage ./.;
      devShell = with pkgs;
        mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rust-analyzer
            rustPackages.clippy
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };

      formatter = pkgs.alejandra;
    });
}

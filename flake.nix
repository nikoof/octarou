{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    pre-commit.url = "github:cachix/pre-commit-hooks.nix";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    pre-commit,
    fenix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnsupportedSystem = true;
      };
      toolchain = with fenix.packages.${system};
        combine [
          minimal.cargo
          minimal.rustc
          targets.x86_64-pc-windows-gnu.latest.rust-std
        ];
      naersk-lib = pkgs.callPackage naersk {};
      libPath = with pkgs;
        lib.makeLibraryPath [
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
    in {
      defaultPackage = with pkgs;
        naersk-lib.buildPackage rec {
          src = ./.;
          pname = "chip8";
          nativeBuildInputs = [
            makeWrapper
          ];

          postInstall = ''
            wrapProgram "$out/bin/${pname}" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

          LD_LIBRARY_PATH = libPath;
        };

      devShell = with pkgs;
        mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;

          buildInputs = [
            cargo
            rustc
            rustfmt
            rust-analyzer
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };

      packages.windowsCross =
        (pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        })
        .buildPackage
        {
          src = ./.;
          pname = "chip8";

          strictDeps = true;
          depsBuildBuild = with pkgs.pkgsCross; [
            mingwW64.stdenv.cc
            mingwW64.windows.pthreads
          ];

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
        };

      formatter = pkgs.alejandra;
      checks.pre-commit-check = pre-commit.lib.${system}.run {
        src = ./.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;
          # clippy.enable = true;
          taplo.enable = true;
        };
      };
    });
}

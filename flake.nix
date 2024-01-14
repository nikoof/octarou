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
        # config.allowUnsupportedSystem = true;
      };
      naerskLib = pkgs.callPackage naersk {};
      toolchain = with fenix.packages.${system};
        combine [
          stable.rustc
          stable.cargo
          stable.rustfmt
          stable.rust-analyzer
          targets.wasm32-unknown-unknown.stable.rust-std
        ];
      toolchainWindows = with fenix.packages.${system};
        combine [
          minimal.cargo
          minimal.rustc
          targets.x86_64-pc-windows-gnu.latest.rust-std
        ];
      libPath = with pkgs;
        lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];
    in {
      packages.default = with pkgs;
        naerskLib.buildPackage rec {
          src = ./.;
          pname = "octarou";
          nativeBuildInputs = [
            makeWrapper
          ];

          postInstall = ''
            wrapProgram "$out/bin/${pname}" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

          LD_LIBRARY_PATH = libPath;
        };

      packages.x86_64-pc-windows-gnu =
        (pkgs.callPackage naersk {
          cargo = toolchainWindows;
          rustc = toolchainWindows;
        })
        .buildPackage
        {
          pname = "octarou";
          src = ./.;
          strictDeps = true;

          depsBuildBuild = with pkgs.pkgsCross; [
            mingwW64.stdenv.cc
            mingwW64.windows.pthreads
          ];
          doCheck = false;

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

      devShell = with pkgs;
        mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;

          buildInputs = with pkgs; [
            toolchain
            trunk
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
    });
}

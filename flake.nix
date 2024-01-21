{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    pre-commit.url = "github:cachix/pre-commit-hooks.nix";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
    pre-commit,
    fenix,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      craneLibWindows = (crane.mkLib pkgs).overrideToolchain toolchainWindows;
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
          stable.rustc
          stable.cargo
          targets.x86_64-pc-windows-gnu.stable.rust-std
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

      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (pkgs.lib.hasSuffix "\.html" path)
          || (pkgs.lib.hasSuffix "\.scss" path)
          || (pkgs.lib.hasInfix "/assets/" path)
          || (craneLib.filterCargoSources path type);
      };

      commonArgs = {
        inherit src;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
        doCheck = false;
      };
    in {
      packages.default = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;

          LD_LIBRARY_PATH = libPath;
        });

      packages.x86_64-pc-windows-gnu = craneLibWindows.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLibWindows.buildDepsOnly {
            inherit src;
            doCheck = false;
          };

          inherit src;

          depsBuildBuild = with pkgs.pkgsCross; [
            mingwW64.stdenv.cc
            mingwW64.windows.pthreads
          ];

          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
        });

      packages.gh-pages = craneLib.buildTrunkPackage (commonArgs
        // {
          inherit (pkgs) wasm-bindgen-cli;
          inherit cargoArtifacts;

          trunkExtraBuildArgs = "--public-url octarou/";

          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        });

      formatter = pkgs.alejandra;
      checks.pre-commit-check = pre-commit.lib.${system}.run {
        src = ./.;
        hooks = {
          alejandra.enable = true;
          taplo.enable = true;
          rustfmt.enable = true;
          # clippy.enable = true;
        };
      };

      devShell = craneLib.devShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;

        packages = with pkgs; [
          trunk
          nodePackages.conventional-changelog-cli
        ];

        LD_LIBRARY_PATH = libPath;
      };
    });
}

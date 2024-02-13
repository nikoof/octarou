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

      cargoArtifactsWasm = craneLib.buildDepsOnly {
        inherit src;
        doCheck = false;
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
      };
    in {
      packages.default = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;

          buildInputs = with pkgs;
            [pkg-config]
            ++ lib.optional (stdenv.isLinux) alsa-lib;

          LD_LIBRARY_PATH = libPath;
        });

      packages.x86_64-pc-windows-gnu = craneLibWindows.buildPackage (commonArgs
        // {
          inherit src;

          depsBuildBuild = with pkgs; [
            pkg-config
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];

          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
        });

      packages.gh-pages = craneLib.buildTrunkPackage (commonArgs
        // {
          inherit (pkgs) wasm-bindgen-cli;
          inherit cargoArtifactsWasm;

          trunkExtraBuildArgs = "--public-url octarou/";

          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        });

      formatter = pkgs.alejandra;
      checks.pre-commit-check = pre-commit.lib.${system}.run {
        src = ./.;
        hooks = {
          alejandra = {
            enable = true;
            excludes = ["doc"];
          };
          taplo.enable = true;
          rustfmt.enable = true;
          # clippy.enable = true;
        };
      };

      devShell = craneLib.devShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;

        inputsFrom = [self.packages.${system}.default];

        packages = with pkgs; [
          trunk
          nodePackages.conventional-changelog-cli
        ];

        LD_LIBRARY_PATH = libPath;
      };

      packages.paper = with pkgs;
        stdenvNoCC.mkDerivation rec {
          name = "ocatrou-paper";
          src = ./doc/paper/src;
          buildInputs = [
            coreutils
            biber
            python311Packages.pygments
            which
            (texlive.combine {
              inherit (texlive) scheme-medium biblatex csquotes minted;
            })
          ];
          phases = ["unpackPhase" "buildPhase" "installPhase"];

          buildPhase = ''
            export PATH="${lib.makeBinPath buildInputs}"
            mkdir -p .cache/texmf-var
            which pygmentize 1> /dev/null
            env TEXMFHOME=.cache TEXMFVAR=.cache/texmf-var \
              latexmk -interaction=nonstopmode -pdf -lualatex -pdflualatex="lualatex -shell-escape %O %S"
          '';

          installPhase = ''
            mkdir -p $out/doc
            cp main.pdf $out/doc/paper.pdf
          '';
        };

      devShells.paper = with pkgs;
        mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          inputsFrom = [self.packages.${system}.paper];
          packages = [
            texlab
            bibtex-tidy
          ];
        };
    });
}

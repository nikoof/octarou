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
  } @ inputs:
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
      in rec
      {
        formatter = pkgs.alejandra;

        defaultPackage = with pkgs;
          naersk-lib.buildPackage rec {
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

        packages.windowsCross =
          (pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          })
          .buildPackage
          {
            src = ./.;
            pname = "octarou";

            strictDeps = true;
            depsBuildBuild = with pkgs.pkgsCross; [
              mingwW64.stdenv.cc
              mingwW64.windows.pthreads
            ];

            CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
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

        checks.pre-commit-check = pre-commit.lib.${system}.run {
          src = ./.;
          hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
            # clippy.enable = true;
            taplo.enable = true;
            chktex.enable = true;
            latexindent.enable = true;
          };
        };

        packages.paper = with pkgs;
          stdenvNoCC.mkDerivation rec {
            name = "ocatrou-paper";
            src = ./doc/paper/src;
            buildInputs = [coreutils texlive.combined.scheme-medium];
            phases = ["unpackPhase" "buildPhase" "installPhase"];

            buildPhase = ''
              export PATH="${lib.makeBinPath buildInputs}"
              mkdir -p .cache/texmf-var
              env TEXMFHOME=.cache TEXMFVAR=.cache/texmf-var \
                latexmk -interaction=nonstopmode -pdf -lualatex \
                ${src}/main.tex
            '';

            installPhase = ''
              mkdir -p $out/doc
              cp main.pdf $out/doc/paper.pdf
            '';
          };

        devShells.paper = with pkgs;
          mkShell {
            inherit (self.checks.${system}.pre-commit-check) shellHook;
            inputsFrom = [packages.paper];
            packages = [texlab];
          };
      });
}

packages.default = craneLib.buildPackage (commonArgs
  // {
    inherit cargoArtifacts;

    buildInputs = with pkgs;
      [pkg-config]
      ++ lib.optional (stdenv.isLinux) alsa-lib;

    LD_LIBRARY_PATH = libPath;
  });

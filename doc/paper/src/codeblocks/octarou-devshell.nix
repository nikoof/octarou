devShell = craneLib.devShell {
  inherit (self.checks.${system}.pre-commit-check) shellHook;

  inputsFrom = [self.packages.${system}.default];

  packages = with pkgs; [
    trunk
    nodePackages.conventional-changelog-cli
  ];

  LD_LIBRARY_PATH = libPath;
};

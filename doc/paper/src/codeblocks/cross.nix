toolchainWindows = with fenix.packages.${system};
  combine [
    stable.rustc
    stable.cargo
    targets.x86_64-pc-windows-gnu.stable.rust-std
  ];

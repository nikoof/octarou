# chip8
![Nix build status](https://github.com/nikoof/chip8/actions/workflows/build_nix.yml/badge.svg)

A cross-platform CHIP-8 interpreter. The interpreter's behaviour is controlled via command-line arguments, and the display is implemented as a GUI. This setup is obviously not ideal, and it will probably change in the future to include a fully-featured GUI for both configuration and rendering.

The interpreter fully implements the original COSMAC VIP CHIP-8 instruction set.
I am currently planning to also add support for SUPER-CHIP (and maybe XO-CHIP).

# Building with Nix
The project can be built using Nix flakes.

## Linux and \*NIXes
```shell
$ # build
$ nix build github:nikoof/chip8
$ ./result/bin/chip8 --help
$ # ...or run it directly
$ nix run github:nikoof/chip8 -- --help
```

## Windows
Building for Windows is done via cross-compilation with Nix on a Linux host.
```shell
$ nix build github:Nikoof/chip8#windowsCross
```
The resulting binary is at `result/bin/chip8.exe`.

# Building manually
Alternatively, you can clone the repo and compile the project natively on all major platforms.
```shell
$ git clone https://github.com/nikoof/chip8 && cd chip8
$ cargo run --release -- --help
```

# Credits
This project is heavily based on [Tobias Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) and [Timendus' Test Suite](https://github.com/Timendus/chip8-test-suite). I am extremely grateful to both authors for these amazing resources.

Other resources used include:
- [Gulrak's Opcode Table](https://chip8.gulrak.net/)
- [Revival Studios ROMs](https://github.com/kripod/chip8-roms)

# CHIP8
This is a CHIP-8 interpreter for my high-school CS final project.

# Building with Nix
## Unix-likes
Run using Nix.
```shell
$ nix run github:Nikoof/chip8 -- --help
```

## Windows
Since Nix doesn't support Windows, you have to cross-compile for Windows on a Linux host.
```shell
$ nix build github:Nikoof/chip8#windowsCross
```
The resulting binary at `result/bin/chip8.exe` can be run with Wine or natively on Windows.

# Building manually
Alternatively, you can pull and compile the project manually on all platforms.
```shell
$ git clone git@github.com:Nikoof/chip8 && cd chip8
$ cargo run --release -- --help
```

# Credits
This project is heavily based on [Tobias Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/).
It also uses [Timendus' Test Suite](https://github.com/Timendus/chip8-test-suite) and [this collection of roms](https://github.com/Timendus/chip8-test-suite).

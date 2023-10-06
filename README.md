# CHIP8
This is a CHIP-8 interpreter for my high-school CS final project.

# Usage
For now, a Nix package isn't available, so you need to use a devshell to compile and run the project.
```shell
$ nix develop github:Nikoof/chip8
$ cargo run --release
```

Alternatively, you can pull and compile the project manually.
```shell
$ git clone git@github.com:Nikoof/chip8 && cd chip8
$ cargo run --release
```

# Credits
This project is heavily based on [Tobias Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/).
It also uses [Timendus' Test Suite](https://github.com/Timendus/chip8-test-suite) and [this collection of roms](https://github.com/Timendus/chip8-test-suite).

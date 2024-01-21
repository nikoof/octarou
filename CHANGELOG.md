# [v1.0.0](https://github.com/nikoof/octarou/compare/v0.2.0...v1.0.0) (2024-01-21)

### Bug Fixes

- make wasm32 version more usable ([a4c8f94](https://github.com/nikoof/octarou/commit/a4c8f941c02a4bb7ec4b442d68e285858ab56b55))
- mangled drawing in hires mode ([3d5aa7d](https://github.com/nikoof/octarou/commit/3d5aa7d37bd64cc35216f0ad6ca21c8d8bd8fe6d))
- name ([b258469](https://github.com/nikoof/octarou/commit/b258469893ad593085fb17ba02a32d728592a1d9))
- NoGlutinConfig error on wayland ([a13bdaf](https://github.com/nikoof/octarou/commit/a13bdaf6d104f96a65edd3cccb4db4f7f5f26c8a))
- **readme:** fix broken links ([1051bdf](https://github.com/nikoof/octarou/commit/1051bdf4f229166f9827499810151713fd0095ee))

### Features

- add egui ([3cbe825](https://github.com/nikoof/octarou/commit/3cbe825d146a4f69d3131931d5f0f0e0a91cc0fa))
- add gui layout ([496d387](https://github.com/nikoof/octarou/commit/496d3870569029e8cf74743ba43f3191a14ca56a))
- add logging ([7fb135a](https://github.com/nikoof/octarou/commit/7fb135a297982e635375a264cbaba0ac1c8369fd))
- add schip support ([77c6ef3](https://github.com/nikoof/octarou/commit/77c6ef3fa86a6f54f6d8b4f675d8f039e77edc21))
- add wasm target ([1511105](https://github.com/nikoof/octarou/commit/15111052066aa19d2200b087d785806a9ffed587))
- better error handling ([6a7a04c](https://github.com/nikoof/octarou/commit/6a7a04c3c3b6f86fbbbb963f77cef1ddec8311e7))
- interpreter and program loading via UI ([80ee83b](https://github.com/nikoof/octarou/commit/80ee83be999757c0baeb12ed4046d94c5ba3df60))
- rearrange ui ([b8ae267](https://github.com/nikoof/octarou/commit/b8ae267412322a6ffc25117610fb658591d916bc))
- use native file dialog (rfd) ([3463c54](https://github.com/nikoof/octarou/commit/3463c543b9cf3cfd04489820ba9d6d9bda4f210f))
- **web:** log to console as well as UI ([12a7886](https://github.com/nikoof/octarou/commit/12a7886478d9a66961958c0a08ee41ca3f4ebcf5))
- window resizing ([447db3e](https://github.com/nikoof/octarou/commit/447db3ed4ddddb0e766a725c354c07a8fe5ca8bd))

# [0.2.0](https://github.com/nikoof/octarou/compare/9b00aaa83382cb089a6f3f1aa9bb268912e1959b...v0.2.0) (2023-11-24)

### Bug Fixes

- added missing 0x2NNN (Call) ([988e7c5](https://github.com/nikoof/octarou/commit/988e7c573aec4a5c89c619653108b3cf2f21ff7a))
- correct(ish) behaviour for 0xFX0A (GetKey) ([b86ba1f](https://github.com/nikoof/octarou/commit/b86ba1f773a4ca92cbffd85db2aa5a7a7434a5c1))
- draw op now correctly sets the flag register ([e8dad44](https://github.com/nikoof/octarou/commit/e8dad449a3c5b393838864bf33e7e22ad0a5a596))
- now passes flag test ([310d15c](https://github.com/nikoof/octarou/commit/310d15c38569be54b9619116e845686443e8b2e6))
- result of Sub now always goes to dest = vx ([644645b](https://github.com/nikoof/octarou/commit/644645b34846f620abf09ef12c8e14bc8dadea66))
- SetIndexFont now works correctly ([455f89a](https://github.com/nikoof/octarou/commit/455f89aa41d776302245b2f5c5b39df56cdc28ad))
- swapped opcodes for left/right shift ([91bf73b](https://github.com/nikoof/octarou/commit/91bf73b65f4611def7535def622de1819ed71e79))
- timing ([f657373](https://github.com/nikoof/octarou/commit/f657373c46ad386ccb818be923e74e1d53ed55f9))

### Features

- add anyhow for error handling ([cd9cc86](https://github.com/nikoof/octarou/commit/cd9cc8603b0b8b8472914ff396523563904aa2fa))
- add interpreter variant option via dynamic dispatch polymorphism ([050fdcf](https://github.com/nikoof/octarou/commit/050fdcf0cc89ecfcadbb5d3a7338fa9b79fb5ced))
- add keypad support ([9b5097d](https://github.com/nikoof/octarou/commit/9b5097db96202330c8e973ba3165f36f1d4e8edf))
- add minifb ([9b00aaa](https://github.com/nikoof/octarou/commit/9b00aaa83382cb089a6f3f1aa9bb268912e1959b))
- add nix derivation for windows cross build ([5f01c0c](https://github.com/nikoof/octarou/commit/5f01c0c3054ae5fc58e58c9a2a8fb5ecd38f6e65))
- add nix package ([c038a19](https://github.com/nikoof/octarou/commit/c038a196db638719d58533fc44ef58e37a8e8ef8))
- add nix package ([91c18f9](https://github.com/nikoof/octarou/commit/91c18f9a3c62a25c0a622391c9e5bd6169a8d92b))
- add Operation variants and opcode decoding ([896316a](https://github.com/nikoof/octarou/commit/896316ac3fd51eb479c06ebb7fc94ef712161b1e))
- add roms submodule and program loading ([f3baedf](https://github.com/nikoof/octarou/commit/f3baedfcf3cc9c92484746d6935ae19cc66568eb))
- add Timendus/chip8-test-suite ([5d39d29](https://github.com/nikoof/octarou/commit/5d39d298f43e4eb00298d3927b2731e0abf5a7ee))
- implement basic operations ([76166e4](https://github.com/nikoof/octarou/commit/76166e4f4dffad9a523fb1750b287542d5b1eb6a))
- implement more operations ([f64bada](https://github.com/nikoof/octarou/commit/f64bada0ead7f0da81fffec05df256fbd02b9099))
- implement timing (badly) ([8fbc857](https://github.com/nikoof/octarou/commit/8fbc85736a47cb55085883e319c5f61e0025f751))
- implemented CLI interface ([1c79cdd](https://github.com/nikoof/octarou/commit/1c79cdd3bc94ef39349d9f17e4af820dbf1684d2))
- skeleton fetch/decode/execute loop ([bc1d941](https://github.com/nikoof/octarou/commit/bc1d9411ff39e536a3b851cdb1277cdc5d1e4d3d))

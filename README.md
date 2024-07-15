# WebBoy

This repo holds the core logic for WebBoy: a Game Boy emulator written in Rust. The code can be compiled down to WebAssembly so it can be played on the web.

This emulator passes all [CPU instruction tests](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs) and [APU tests](https://github.com/retrio/gb-test-roms/tree/master/dmg_sound) from Blargg's test ROM collection.

## How to Use

1. Clone [webboy-client](https://github.com/smparsons/webboy-client) to your local machine. The project should live under the same directory as webboy-core.
2. To build the project, simply run `cargo build`.
3. To compile the implementation to WebAssembly, you will first need to install wasm-pack with the command `cargo install wasm-pack`. Then, run `sh ./build.sh` to generate the Javascript binding code in the webboy-client project.

## Test Suite

This project holds a fairly extensive test suite, as the bulk of the logic was designed using a TDD approach. There are a lot of tests that exercise CPU opcodes, and basic tests that exercise the GPU. Run `cargo test` to run the test suite.

## Supported Features

This emulator is still a work in progress and not all features are supported.

| Feature           | Supported |
| ----------------- | --------- |
| CPU               | ✅        |
| Basic Graphics    | ✅        |
| Audio             | ✅        |
| Color Support     | ❌        |
| GameShark Support | ❌        |

### MBC Support

At the moment, only MBC1 is supported.

| Type | Supported |
| ---- | --------- |
| MBC1 | ✅        |
| MBC2 | ❌        |
| MBC3 | ❌        |
| MBC4 | ❌        |
| MBC5 | ❌        |
| MBC6 | ❌        |
| MBC7 | ❌        |

## Helpful Resources

I described some of the resources I used to build this emulator in a [blog post](https://samthecoder.com/must-have-resources-for-building-a-gameboy-emulator).

For convenience, here is a list of the resources I used:

1. [Gameboy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
2. [Pan Docs](https://gbdev.io/pandocs/)
3. [Blargg's Test ROM Collection](https://github.com/retrio/gb-test-roms)
4. [Gameboy Doctor](https://github.com/robert/gameboy-doctor)
5. [Imran Nazar's Gameboy Emulator Tutorial](https://imrannazar.com/series/gameboy-emulation-in-javascript)

## Future Plans

Eventually, I plan to test this emulator using Blargg's [CPU instruction timing tests](https://github.com/retrio/gb-test-roms/tree/master/instr_timing) as well as the [Mooneye test ROM collection](https://github.com/Gekkio/mooneye-test-suite).

This emulator can reliably run Tetris and Tic-Tac-Toe, however I need to test it with other games as well.

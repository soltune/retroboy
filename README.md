# Retro Boy

This repo holds the core logic for Retro Boy: a Game Boy emulator written in Rust.

The code can also be compiled down to WebAssembly so it can be played on the web.

This emulator passes the following test suites from Blargg's test ROM collection:

1. [CPU instruction tests](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs)
2. [CPU instruction timing tests](https://github.com/retrio/gb-test-roms/tree/master/instr_timing)
3. [Memory timing tests](https://github.com/retrio/gb-test-roms/tree/master/mem_timing)
4. [Memory timing tests 2](https://github.com/retrio/gb-test-roms/tree/master/mem_timing-2)
5. [APU tests (DMG)](https://github.com/retrio/gb-test-roms/tree/master/dmg_sound)

## How to Compile to WebAssembly

To compile the implementation to WebAssembly, you will first need to install wasm-pack with the command `cargo install wasm-pack` if you haven't done so already. Then, run `sh ./build-wasm.sh` to build the core project and generate the Javascript binding code in the web frontend directory.

## Web Frontend

The web frontend for this emulator is a React/TypeScript app designed with Material UI. It is located in the frontends/web folder. The UI provides the ability to load a ROM as well as play, pause, or reset the emulator. It also provides a fullscreen mode.

To run the web frontend:

1. Compile the Rust code to WebAssembly and generate the Javascript binding code as described in the "How to Compile to WebAssembly" section.
2. When the binding code is generated, it will be added to the frontends/web/src/core directory.
3. Run `yarn install` in the frontends/web directory to install all dependencies.
4. Run `yarn start` in the same directory to run the application locally.

## Screenshots

<p float="left">
  <img src="screenshots/kirby.png" />
  <img src="screenshots/pacman.png" />
  <img src="screenshots/super-mario-land.png" />  
  <img src="screenshots/tetris.png" />
</p>

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

Eventually, I plan to test this emulator using [Mooneye test ROM collection](https://github.com/Gekkio/mooneye-test-suite).

This emulator can reliably run Tetris and Tic-Tac-Toe, however I need to test it with other games as well.

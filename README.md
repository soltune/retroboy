# WebBoy

WebBoy is a Game Boy emulator written in Rust. The code can be compiled down to WebAssembly so it can be played on the web.

## How to Use

1. Clone [webboy-client](https://github.com/smparsons/webboy-client) to your local machine. The project should live under the same directory as webboy-core.
2. To build the project, simply run `cargo build`.
3. To compile the implementation to WebAssembly, you will first need to install wasm-pack with the command `cargo install wasm-pack`. Then, run `sh ./build.sh` to generate the Javascript binding code in the webboy-client project.

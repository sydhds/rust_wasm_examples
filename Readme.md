## Intro

Rust + wasm examples

## Examples

### Rust

* rust_to_wasm_the_hard_way_01:
  * from: https://surma.dev/things/rust-to-webassembly/ - part: "add functions"
  * build:
    * `cargo build --target=wasm32-unknown-unknown --release`
  * inspect wasm file (install wasm2wat with: `sudo apt install wabt`):
    *  `wasm2wat ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_01.wasm`
  * stip wasm file manually:
    * `wasm-strip ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_01.wasm`
  * inspect section (Note: this is removed by strip if unused):
    * `wasm-objdump -s -j MySection1 ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_01.wasm`
  * run wasm file using Nodejs:
    * `cd nodejs_tester && node index.js`
* rust_to_wasm_the_hard_way_02:
  * from: https://surma.dev/things/rust-to-webassembly/ - part: "wasm size optim"
  * build:
    * `cargo build --target=wasm32-unknown-unknown --release`
  * strip but keep the section name:
    * `llvm-strip --keep-section=name ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_02.wasm`
  * inspect wasm with twiggy (`cargo install twiggy`):
    * `twiggy top ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_02.wasm`
  * optim more (sudo apt install binaryen):
    * `wasm-opt -O3 -o ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_02.optim.wasm ../target/wasm32-unknown-unknown/release/rust_wasm_the_hard_way_02.wasm`
* rust_to_wasm_the_hard_way_03:
  * from: https://surma.dev/things/rust-to-webassembly/ - part: "memory allocator"
  * build with custom allocator:
    * cargo build --target=wasm32-unknown-unknown --features custom_alloc --release
  * build with [lol_alloc](https://github.com/Craig-Macomber/lol_alloc)
    * cargo build --target=wasm32-unknown-unknown --features lol_alloc --release
* rust_to_wasm_the_hard_way_04:
  * from: https://surma.dev/things/rust-to-webassembly/ - part: "wasm-bindgen"
  * build:
    * wasm-pack build --target nodejs
  * run wasm file using Nodejs
    * `cd nodejs_tester && node index.js`

* rust_wasmer_01: run wasm file using [wasmer]() TODO
* rust_wasmer_02: run minimalist C library in [wasmer]() TODO
* rust_wasmer_02: run real example of C library in [wasmer]() TODO 

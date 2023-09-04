# `po6`

A C POSIX library in Rust meant for building C libraries in environments
without one (eg. `wasm32-unknown-unknown` or Windows).

Makes it possibles to build C libraries with `wasm-bindgen`, which is not
compatible with `emscripten`.

## Usage

1. Install it to your `build-dependencies`:
   ```console
   $ cargo add po6 --build
   ```
2. Use it in your `build.rs` by adding the output of `po6::build` to your include paths:
   ```rust
   cc::Build::new()
     .includes(po6::build()?)
   ```
3. Use it in your `lib.rs` by including the generated runtime file:
   ```rust
   mod po6 {
     include!(concat!(env!("OUT_DIR"), "/po6.rs"));
   }
   ```

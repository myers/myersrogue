# myersrogue

I'm following along in Hands On Rust.  It's been a fun way to learn and practice rust.

## notes

<https://opengameart.org/content/dungeon-crawl-32x32-tiles>

## Build for web

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/myersrogue.wasm --out-dir ./web --no-modules --no-typescript
```

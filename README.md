# command-battle

# wasm

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm --out-dir wasm/release --target web target/wasm32-unknown-unknown/release/command-battle.wasm
basic-http-server web/wasm
```

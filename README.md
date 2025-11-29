# command-battle

# wasm

```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_example   --out-dir examples/wasm/target   --target web target/wasm32-unknown-unknown/debug/command-battle.wasm
basic-http-server examples/wasm
```

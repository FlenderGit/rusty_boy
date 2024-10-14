# RustyBoy : WASM


Version debug
```
cargo build --target wasm32-unknown-unknown
wasm-bindgen ..\..\target\wasm32-unknown-unknown\debug\web.wasm --target web --out-dir .\generated\
```

Version Release
```
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ..\..\target\wasm32-unknown-unknown\release\web.wasm --target web --out-dir .\generated\
```
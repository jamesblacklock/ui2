cargo build --lib --target=wasm32-unknown-unknown --release
cd pkg_web
mkdir -p ../dist
cargo run -- ../target/wasm32-unknown-unknown/release/runtime.wasm ../dist/runtime_wasm.js

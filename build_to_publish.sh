cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/cart_snipped.wasm
wasm-opt target/wasm32-unknown-unknown/release/cart_snipped.wasm -Oz --zero-filled-memory --strip-producers --dce --output disk-0-madness.wasm

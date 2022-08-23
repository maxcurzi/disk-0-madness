cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/cart_snipped.wasm
# w4 run target/wasm32-unknown-unknown/release/cart_snipped.wasm
wasm-opt target/wasm32-unknown-unknown/release/cart_snipped.wasm -Oz --zero-filled-memory --strip-producers --dce --output target/wasm32-unknown-unknown/release/cart_snipped_optimized.wasm
w4 run target/wasm32-unknown-unknown/release/cart_snipped_optimized.wasm
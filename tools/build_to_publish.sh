cargo build --release

wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm > target/wasm32-unknown-unknown/release/cart_snipped.wasm

mkdir publish
wasm-opt target/wasm32-unknown-unknown/release/cart_snipped.wasm -Oz --zero-filled-memory --strip-producers --dce --output publish/disk-0-madness.wasm

w4 bundle publish/disk-0-madness.wasm --title "disk-0 MADNESS" \
    --icon-file assets/icons/html_icon.png \
    --windows publish/disk-0-MADNESS-windows.exe \
    --mac publish/disk-0-MADNESS-mac \
    --linux publish/disk-0-MADNESS-linux \
    --html publish/disk-0-madness.html \

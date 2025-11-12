test-members:
    cargo build -p test-lib --target wasm32-unknown-unknown --release
    cargo build -p test-fetch-post --target wasm32-unknown-unknown --release
    cargo build -p test-readfs --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/test_*.wasm test-units
    cargo run -p bolter --release

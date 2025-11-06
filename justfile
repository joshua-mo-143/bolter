test-members:
    cargo build -p test-lib --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/test_lib.wasm test-units
    cargo run -p bolter --release

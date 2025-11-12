test-members:
    cargo build -p test-hello --target wasm32-unknown-unknown --release
    cargo build -p test-fetch-post --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readdir --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readfile --target wasm32-unknown-unknown --release
    cargo build -p test-fs-writefile --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/test_*.wasm test-units
    cargo run -p bolter --release --bin basic

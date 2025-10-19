test-members:
    cargo build -p test-member --bins --target wasm32-wasip1
    cargo build -p test-lib --target wasm32-wasip1
    cp target/wasm32-wasip1/debug/test-member1.wasm test-units
    cp target/wasm32-wasip1/debug/test-member2.wasm test-units
    cp target/wasm32-wasip1/debug/test_lib.wasm test-units
    cargo run -p filigree

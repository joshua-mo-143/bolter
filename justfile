test-members:
    cargo build -p test-lib --target wasm32-wasip1 --release
    cp target/wasm32-wasip1/release/test_lib.wasm test-units
    cargo run -p bolter --release

test-members-full:
    cargo build -p test-member --bins --target wasm32-wasip1 --release
    cargo build -p test-lib --target wasm32-wasip1 --release
    cp target/wasm32-wasip1/release/test-member1.wasm test-units
    cp target/wasm32-wasip1/release/test-member2.wasm test-units
    cp target/wasm32-wasip1/release/test_lib.wasm test-units
    cargo run -p bolter --release

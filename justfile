# Build all WASM test modules (done in release mode) and send them to `test-units` folder
# Aforementioned test modules are automatically gitignored so you don't need to worry about accidentally
# committing them!
build:
    cargo build -p test-hello --target wasm32-unknown-unknown --release
    cargo build -p test-fetch-post --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readdir --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readfile --target wasm32-unknown-unknown --release
    cargo build -p test-fs-writefile --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/test_*.wasm test-units

# Build all test modules and run the binary to test them all
build-test:
    just build
    cargo run -p bolter --release --bin test_all_modules

# Run the Bolter example binary web server
server:
    cargo run -p bolter --bin webserver

# Build all test modules and run the Bolter example binary web server
build-server:
    just build
    cargo run -p bolter --bin webserver

query prompt:
    curl -X POST -H 'content-type: application/json' localhost:8000 -d '{"prompt":"{{prompt}}"}' | jq -r '.response'

build:
    cargo build -p test-hello --target wasm32-unknown-unknown --release
    cargo build -p test-fetch-post --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readdir --target wasm32-unknown-unknown --release
    cargo build -p test-fs-readfile --target wasm32-unknown-unknown --release
    cargo build -p test-fs-writefile --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/test_*.wasm test-units

build-test:
    just build
    cargo run -p bolter --release --bin test_all_modules

server:
    cargo run -p bolter --bin webserver

build-server:
    just build
    cargo run -p bolter --bin webserver

query prompt:
    curl -X POST -H 'content-type: application/json' localhost:8000 -d '{"prompt":"{{prompt}}"}' | jq -r '.response'

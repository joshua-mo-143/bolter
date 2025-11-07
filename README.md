## Bolter
Bolter is an experimental agent runtime using sandboxed tool execution (built on top of [`rig`](https://www.github.com/0xplaygrounds/rig)).

There are currently several components to this library:
- The main program itself (currently doesn't do that much, just simulates a conversation with an LLM while calling into the tool sandboxing)
- A crate to hold macros primarily to help with writing tool functions in WASM modules
- A library that compiles to WASM (`test-lib`)

## How to Use
Before you use this, you will need [`wasmtime`](https://docs.wasmtime.dev/cli-install.html) installed as well as the Rust programming language.
You will also need an OpenAI API key.

Currently, there is a single `justfile` command you can run that will build and then run the main program.

Currently the main program will expect an `OPENAI_API_KEY` environment variable (this is where your OpenAI API key goes) and will log the output of the conversation chain with the LLM while calling the test library.

## Disclaimer
Although I am the maintainer of [`rig`](https://www.github.com/0xplaygrounds/rig) which this project uses (and is likely to use heavily in the future, package deprecation notwithstanding), this project is totally unaffiliated with the aforementioned repo and is entirely a personal project.

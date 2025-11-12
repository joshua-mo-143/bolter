## Bolter
Bolter is an experimental agent runtime using sandboxed tool execution (built on top of [`rig`](https://www.github.com/0xplaygrounds/rig)).

There are currently several components to this library:
- The `bolter` library itself
  - This library also contains two binaries you can run: one for simply just testing the modules work, and a web server you can send POST requests to.
- A macro-based crate layer (to help with writing tools... since handling raw pointers is generally not good DX)
- Several WASM-based test modules that can be loaded into the runtime from a config file (see `test-units` folder)
- a `justfile` with several different helpful commands you can try

## How to Use
Before you use this, you will need [`wasmtime`](https://docs.wasmtime.dev/cli-install.html) installed as well as the Rust programming language.
You will also need an OpenAI API key.

Currently, there is a single `justfile` command you can run that will build and then run the main program. You MUST use the `just` commands from the top level of the directory (since otherwise some of the paths may break and show "file/directory not found" errors).

Currently the main program will expect an `OPENAI_API_KEY` environment variable (this is where your OpenAI API key goes) and will log the output of the conversation chain with the LLM while calling the test library.

## Disclaimer
Although I am the maintainer of [`rig`](https://www.github.com/0xplaygrounds/rig) which this project uses (and is likely to use heavily in the future, package deprecation notwithstanding), this project is totally unaffiliated with the aforementioned repo and is entirely a personal project.

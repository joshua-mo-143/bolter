## Bolter
Bolter is an experimental agent runtime using sandboxed tool execution (built on top of [`rig.`](https://www.github.com/0xplaygrounds/rig))

There are currently several components to this library:
- The main program itself (currently doesn't do that much, just simulates a conversation with an LLM while calling into the tool sandboxing)
- A crate to hold macros primarily to help with writing tool functions in WASI modules
- A library that compiles to WASI (`test-lib`)
- A binary that compiles to WASI (`test-member`). This compiles to two binaries - `test-member1` and `test-member2`.

## How to Use
Currently, there is a single `justfile` command you can run that will build and then run the main program.

Currently the main program will expect an `OPENAI_API_KEY` environment variable and will log the output of the conversation chain with the LLM while calling the test library.

## Disclaimer
Although I am the maintainer of [`rig`](https://www.github.com/0xplaygrounds/rig) which this project uses (and is likely to use heavily in the future, package deprecation notwithstanding), this project is totally unaffiliated with the aforementioned repo and is entirely a personal project.

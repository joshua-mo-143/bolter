use bolter::agent::AgentRuntimeExt;
use bolter::wasm::runtime::WasmRuntime;
use rig::client::CompletionClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = WasmRuntime::with_modules_from_file("test-units/test_config.json")?;

    let mut agent = rig::providers::openai::Client::from_env()
        .agent("gpt-4o")
        .preamble("You are a helpful agent.

            You have several tools that you can call should users need help with reading their filesystem, creating files, or anything else.
            All tools are sandboxed in WASM modules, so some tools may not be given correct permissions.
            In the case that you get a permissions error, please inform the user that they need to ensure permissions are correctly set.
            This may also happen if they have installed a new tool in the case that it is malicious and is trying to do things other than what it is intended to do.")
        .build()
        .with_wasm_runtime(runtime);

    let prompt = "Please use my each of my tools to help me verify that \
        this demo works, and return what all of them say.";

    println!("Prompt: {prompt}");

    let res = agent.prompt(prompt).await?;
    println!("Final response:\n{res}");

    Ok(())
}

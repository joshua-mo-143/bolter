use bolter::agent::AgentRuntimeExt;
use bolter::wasm::runtime::WasmRuntime;
use rig::client::{CompletionClient, ProviderClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = WasmRuntime::with_modules_from_file("test-units/test_config.json")?;

    let mut agent = rig::providers::openai::Client::from_env()
        .agent("gpt-4o")
        .preamble("You are a helpful agent equipped with WASM module based tools.")
        .build()
        .with_wasm_runtime(runtime);

    let prompt = "Please use my each of my tools to help me verify that \
        this demo works, and return what all of them say.";

    println!("Prompt: {prompt}");

    let res = agent.prompt(prompt).await?;
    println!("Final response:\n{res}");

    Ok(())
}

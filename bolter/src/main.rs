use crate::agent::AgentRuntimeExt;
use rig::client::{CompletionClient, ProviderClient};
use wasm::runtime::WasmRuntime;

pub mod agent;
pub mod config;
pub mod wasm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = WasmRuntime::new()?;

    let config = config::Config::from_file("test-units/test_config.json");

    for binary in config.data {
        runtime.add_module(binary)?;
    }

    let mut agent = rig::providers::openai::Client::from_env()
        .completion_model("gpt-4o")
        .with_wasm_runtime(runtime);

    let prompt = "Please use my WASM tool twice to help me verify that this demo works, and return what it says both times.";

    println!("Prompt: {prompt}");

    let res = agent.prompt(prompt).await?;
    println!("Final response: {res}");

    Ok(())
}

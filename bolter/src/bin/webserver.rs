use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post, serve};
use bolter::{agent::AgentRuntimeExt, agent::AgentWithRuntime, wasm::runtime::WasmRuntime};
use rig::client::CompletionClient;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};

#[derive(Clone)]
struct AppState {
    agent: Arc<Mutex<AgentWithRuntime<ResponsesCompletionModel>>>,
}

#[derive(Serialize)]
struct Response {
    response: String,
}

#[derive(Deserialize)]
struct Request {
    prompt: String,
}

async fn llm_request(
    State(AppState { agent }): State<AppState>,
    Json(Request { prompt }): Json<Request>,
) -> Result<Json<Response>, String> {
    let mut lock = agent.lock().await;
    let response = lock.prompt(prompt).await.map_err(|x| x.to_string())?;

    Ok(Json(Response { response }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = WasmRuntime::with_modules_from_file("test-units/test_config.json")?;

    let agent = rig::providers::openai::Client::from_env()
        .agent("gpt-5-mini")
        .preamble("You are a helpful agent equipped with WASM module based tools.")
        .build()
        .with_wasm_runtime(runtime);

    let state = AppState {
        agent: Arc::new(Mutex::new(agent)),
    };

    let rtr = Router::new()
        .route("/", post(llm_request))
        .with_state(state);
    let tcp = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Starting webserver at 0.0.0.0:8000...");

    serve(tcp, rtr).await.unwrap();

    Ok(())
}

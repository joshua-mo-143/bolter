use rig::{
    OneOrMany,
    agent::Text,
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, CompletionRequest, ToolDefinition},
    message::{AssistantContent, Message, ToolResult, ToolResultContent, UserContent},
};
use wasmtime::{Engine, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, p1::WasiP1Ctx};

use crate::config::ModuleKind;

pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let mut linker_wasi = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker_wasi, |ctx| ctx)?;
    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(".", ".", DirPerms::all(), FilePerms::all())
            .unwrap()
            .build_p1(),
    );

    let config = config::Config::from_file("test-units/test_config.json");

    let mut tools: Vec<(
        String,
        (
            ToolDefinition,
            Module,
            Instance,
            TypedFunc<(i32, i32, i32, i32), i32>,
        ),
    )> = Vec::new();

    for binary in config.data {
        let module = Module::from_file(&engine, &binary.path)?;
        let instance = linker_wasi.instantiate(&mut store, &module)?;

        match binary.module_type {
            ModuleKind::Tool => {
                let tooldef = get_tool_definition(&instance, &mut store)?;
                let tooldef = ToolDefinition {
                    name: binary.title.clone(),
                    description: binary.description,
                    parameters: serde_json::from_str(&tooldef).unwrap(),
                };
                let toolcall: TypedFunc<(i32, i32, i32, i32), i32> =
                    instance.get_typed_func(&mut store, "run_tool")?;

                tools.push((binary.title, (tooldef, module, instance, toolcall)));
            }
            _ => {}
        }
    }

    let completion_model = rig::providers::openai::Client::from_env().completion_model("gpt-4o");

    let tooldefs: Vec<ToolDefinition> = tools.iter().map(|x| x.1.0.clone()).collect();

    let mut chat_history: Vec<Message> = Vec::new();

    let prompt =
        "Please use my WASI tool to help me verify that this demo works, and return what it says.";

    chat_history.push(prompt.into());

    let res = completion_model.completion_request("Please use my test_lib tool to help me verify that this demo works, and return what it says.")
        .tools(tooldefs.clone())
        .preamble("You are a helpful agent".into())
        .tool_choice(rig::message::ToolChoice::Required)
        .send()
        .await
        .unwrap();

    match res.choice.first() {
        AssistantContent::Text(text) => {
            println!("Response: {text}")
        }
        AssistantContent::ToolCall(tc) => {
            chat_history.push(Message::Assistant {
                id: None,
                content: OneOrMany::one(AssistantContent::ToolCall(tc.clone())),
            });
            let Some((_, (_, _, instance, func))) = tools.iter().find(|x| x.0 == tc.function.name)
            else {
                return Err("Attempted to call a tool that doesn't exist".into());
            };

            let text = run_wasm_tool(instance, &mut store, func.to_owned(), tc.function.arguments)?;

            chat_history.push(Message::User {
                content: OneOrMany::one(UserContent::ToolResult(ToolResult {
                    id: tc.id,
                    call_id: tc.call_id,
                    content: OneOrMany::one(ToolResultContent::Text(Text { text })),
                })),
            });
        }
        _ => {}
    }

    let completion_request = CompletionRequest {
        preamble: None,
        chat_history: OneOrMany::many(chat_history).unwrap(),
        documents: Vec::new(),
        tools: tooldefs,
        temperature: None,
        max_tokens: None,
        tool_choice: None,
        additional_params: None,
    };

    let res = completion_model
        .completion(completion_request)
        .await
        .unwrap();

    println!("{:?}", res.choice.first());

    Ok(())
}

fn run_wasm_tool(
    instance: &Instance,
    mut store: &mut Store<WasiP1Ctx>,
    func: TypedFunc<(i32, i32, i32, i32), i32>,
    args: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("no memory export");
    let tool_input = serde_json::to_string(&args).unwrap();
    let input_bytes = tool_input.as_bytes();
    let input_ptr = 1024u32;
    let input_len = input_bytes.len() as u32;
    let output_ptr = input_ptr + input_len;
    let output_cap = 1024u32;

    let memory_size = memory.data_size(&store);
    let required_memory = (output_ptr + output_cap) as usize;

    if required_memory > memory_size {
        let extra_pages = ((required_memory - memory_size) / (64 * 1024)) + 1;
        memory.grow(&mut store, extra_pages as u64)?;
    }

    memory.write(&mut store, input_ptr as usize, &input_bytes)?;

    let bytes_written = func.call(
        &mut store,
        (
            input_ptr as i32,
            input_len.try_into().unwrap(),
            output_ptr as i32,
            output_cap as i32,
        ),
    )?;

    let mut buf = vec![0u8; bytes_written as usize];
    memory.read(&mut store, output_ptr as usize, &mut buf)?;

    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}

pub fn get_tool_definition(
    instance: &Instance,
    mut store: &mut Store<WasiP1Ctx>,
) -> Result<String, Box<dyn std::error::Error>> {
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("no memory export");
    let toolcall: TypedFunc<(i32, i32), i32> =
        instance.get_typed_func(&mut store, "tool_definition")?;

    let output_ptr = 1024u32;
    let output_cap = 4096u32;

    let memory_size = memory.data_size(&store);
    let required_memory = (output_ptr + output_cap) as usize;

    if required_memory > memory_size {
        let extra_pages = ((required_memory - memory_size) / (64 * 1024)) + 1;
        memory.grow(&mut store, extra_pages as u64)?;
    }

    let bytes_written = toolcall.call(&mut store, (output_ptr as i32, output_cap as i32))?;

    let mut buf = vec![0u8; bytes_written as usize];
    memory.read(&mut store, output_ptr as usize, &mut buf)?;

    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let engine = Engine::default();
//     let mut linker_wasi = Linker::new(&engine);
//     wasmtime_wasi::p1::add_to_linker_sync(&mut linker_wasi, |ctx| ctx)?;
//     let mut store = Store::new(
//         &engine,
//         WasiCtxBuilder::new()
//             .inherit_stdio()
//             .preopened_dir(".", ".", DirPerms::all(), FilePerms::all())
//             .unwrap()
//             .build_p1(),
//     );

//     let config = config::Config::from_file("test-units/test_config.json");

//     let mut tooldefs: Vec<(String, (ToolDefinition, ))> = Vec::new();

//     for binary in config.data {
//         let module = Module::from_file(&engine, &binary.path)?;

//         if let Some(thing) = module.get_export("tool_metadata") {
//             println!("{thing:?}");
//         }
//         let instance = linker_wasi.instantiate(&mut store, &module)?;

//         match binary.module_type {
//             ModuleKind::Binary => {
//                 let start: TypedFunc<(), ()> =
//                     instance.get_typed_func(&mut store, "_start").unwrap();
//                 start.call(&mut store, ())?;
//             }
//             ModuleKind::Tool => {
//                 let tooldef = get_tool_definition(&instance, &mut store)?;
//                 let toolcall: TypedFunc<(i32, i32, i32, i32), i32> =
//                     instance.get_typed_func(&mut store, "run_tool")?;

//                 tooldefs.push(serde_json::from_str(&tooldef).unwrap());

//                 println!("Tooldef: {tooldef}");
//                 let result = run_wasm_tool(&instance, &mut store)?;

//                 println!("Got from wasm: {result}");
//             }
//         }
//     }

//     Ok(())
// }

use wasmtime::{Engine, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, p1::WasiP1Ctx};

use crate::config::ModuleKind;

pub mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    for binary in config.data {
        let module = Module::from_file(&engine, &binary.path)?;

        if let Some(thing) = module.get_export("tool_metadata") {
            println!("{thing:?}");
        }
        let instance = linker_wasi.instantiate(&mut store, &module)?;

        match binary.module_type {
            ModuleKind::Binary => {
                let start: TypedFunc<(), ()> =
                    instance.get_typed_func(&mut store, "_start").unwrap();
                start.call(&mut store, ())?;
            }
            ModuleKind::Tool => {
                let tooldef = get_tool_definition(&instance, &mut store)?;
                println!("Tooldef: {tooldef}");
                let result = run_wasm_tool(&instance, &mut store)?;

                println!("Got from wasm: {result}");

                let result = run_wasm_tool(&instance, &mut store)?;

                println!("Got from wasm: {result}");
            }
        }
    }

    Ok(())
}

fn run_wasm_tool(
    instance: &Instance,
    mut store: &mut Store<WasiP1Ctx>,
) -> Result<String, Box<dyn std::error::Error>> {
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("no memory export");
    let toolcall: TypedFunc<(i32, i32, i32, i32), i32> =
        instance.get_typed_func(&mut store, "run_tool")?;

    let tool_input = serde_json::json!({"bar": "Hello world!"});
    let tool_input = serde_json::to_string(&tool_input).unwrap();
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

    let bytes_written = toolcall.call(
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
    let output_cap = 1024u32;

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

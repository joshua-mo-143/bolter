use wasmtime::{Engine, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, p1::WasiP1Ctx};

pub mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let mut linker_wasi = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker_wasi, |ctx| ctx)?;
    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(".", "eep", DirPerms::all(), FilePerms::all())
            .unwrap()
            .build_p1(),
    );

    let config = config::Config::from_file("test-units/test_config.json");

    for binary in config.data {
        let module = Module::from_file(&engine, &binary.path)?;
        let instance = linker_wasi.instantiate(&mut store, &module)?;
        for export in instance.exports(&mut store) {
            println!("{}", export.name());
        }

        let start: TypedFunc<(), ()> = instance.get_typed_func(&mut store, "_start").unwrap();
        start.call(&mut store, ())?;

        let result = run_wasm_tool(&instance, &mut store)?;

        println!("Got from wasm: {result}");
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

    let input_bytes = b"Josh";
    let input_ptr = memory.data_size(&store) as u32;
    let input_len = input_bytes.len() as u32;

    let required_pages = ((input_ptr + input_len) as usize / (64 * 1024)) as u64 + 1;
    memory.grow(&mut store, required_pages)?;

    memory.write(&mut store, input_ptr as usize, input_bytes)?;

    let output_cap = 64u32;
    let output_ptr = input_ptr + input_len;

    let required_pages = ((input_ptr + input_len) as usize / (64 * 1024)) as u64 + 1;
    memory.grow(&mut store, required_pages)?;

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

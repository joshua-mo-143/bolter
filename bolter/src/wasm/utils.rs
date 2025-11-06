use wasmtime::{Instance, Store, TypedFunc};

pub(crate) fn run_wasm_tool(
    instance: &Instance,
    mut store: &mut Store<()>,
    func: &TypedFunc<(i32, i32, i32, i32), i32>,
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

    memory.write(&mut store, input_ptr as usize, input_bytes)?;

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
    mut store: &mut Store<()>,
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

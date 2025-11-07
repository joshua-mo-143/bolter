use schemars::JsonSchema;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn fetch_url(ptr: i32, len: i32, output_ptr: *mut u8);
    fn fetch_url_post(ptr: *const u8, len: i32, output_ptr: *mut u8);
}

#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: i32,
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct Foo {
    pub bar: String,
}

#[macros::wasi_tool]
pub fn my_tool(input: Foo) -> String {
    let thing = serde_json::to_vec(&input).unwrap();
    let Foo { bar } = input;

    let result = unsafe {
        let mut output_buf = Buffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        let output_ptr = &mut output_buf as *mut Buffer as *mut u8;
        fetch_url(thing.as_ptr() as i32, thing.len() as i32, output_ptr);

        let response_slice =
            std::slice::from_raw_parts(output_buf.ptr as *const u8, output_buf.len as usize);

        String::from_utf8_lossy(response_slice).to_string()
    };

    // let result = unsafe {
    //     let mut output_buf = Buffer {
    //         ptr: std::ptr::null_mut(),
    //         len: 0,
    //     };
    //     let output_ptr = &mut output_buf as *mut Buffer as *mut u8;
    //     fetch_url_post(thing.as_ptr(), thing.len() as i32, output_ptr);

    //     let response_slice =
    //         std::slice::from_raw_parts(output_buf.ptr as *const u8, output_buf.len as usize);

    //     String::from_utf8_lossy(response_slice).to_string()
    // };
    format!("Hello {result}")
}

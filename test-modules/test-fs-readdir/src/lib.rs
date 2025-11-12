use schemars::JsonSchema;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn read_dir(ptr: *const u8, len: i32, output_ptr: *mut u8);
}

#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: i32,
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct Foo {
    /// The directory to read.
    pub dir_path: String,
}

#[macros::wasi_tool]
pub fn my_tool(input: Foo) -> String {
    let dir_path_bytes = input.dir_path.into_bytes();
    let (len, ptr) = (dir_path_bytes.len(), dir_path_bytes.as_ptr());

    let result = unsafe {
        let mut output_buf = Buffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        let output_ptr = &mut output_buf as *mut Buffer as *mut u8;
        read_dir(ptr, len as i32, output_ptr);

        let response_slice =
            std::slice::from_raw_parts(output_buf.ptr as *const u8, output_buf.len as usize);

        String::from_utf8_lossy(response_slice).to_string()
    };

    result
}

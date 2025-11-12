use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn write_file(ptr: *const u8, len: i32, output_ptr: *mut u8);
}

#[repr(C)]
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: i32,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct WriteFileRequest {
    /// The file path to write to.
    path: String,
    /// The contents to write (as a raw string)
    contents: String,
}

#[macros::wasi_tool]
pub fn my_tool(input: WriteFileRequest) -> String {
    let req = serde_json::to_vec(&input).unwrap();
    let (len, ptr) = (req.len(), req.as_ptr());

    let result = unsafe {
        let mut output_buf = Buffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        let output_ptr = &mut output_buf as *mut Buffer as *mut u8;
        write_file(ptr, len as i32, output_ptr);

        let response_slice =
            std::slice::from_raw_parts(output_buf.ptr as *const u8, output_buf.len as usize);

        String::from_utf8_lossy(response_slice).to_string()
    };

    result
}

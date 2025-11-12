use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[link(wasm_import_module = "env")]
unsafe extern "C" {
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

#[derive(Deserialize, Serialize)]
struct HttpRequest {
    pub body: Vec<u8>,
    pub headers: BTreeMap<String, PlaintextOrSecret>,
    pub url: String,
}

impl HttpRequest {
    pub fn to_pointer(self) -> (*const u8, i32) {
        let bytes = serde_json::to_vec(&self).unwrap();
        let input_len = bytes.len() as i32;
        let bytes_ptr = bytes.as_ptr();

        (bytes_ptr, input_len)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "valueType", rename_all = "lowercase", content = "value")]
enum PlaintextOrSecret {
    Plaintext(String),
    Secret(String),
}

#[macros::wasi_tool]
pub fn my_tool(input: Foo) -> String {
    let body = serde_json::to_vec(&input).unwrap();
    let mut headers = BTreeMap::new();
    headers.insert(
        "content-type".to_string(),
        PlaintextOrSecret::Plaintext("application/json".to_string()),
    );

    let (req_ptr, req_size) = HttpRequest {
        body,
        headers,
        url: "https://httpbin.org/post".to_string(),
    }
    .to_pointer();

    let post_result = unsafe {
        let mut output_buf = Buffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        let output_ptr = &mut output_buf as *mut Buffer as *mut u8;
        fetch_url_post(req_ptr, req_size, output_ptr);

        let response_slice =
            std::slice::from_raw_parts(output_buf.ptr as *const u8, output_buf.len as usize);

        String::from_utf8_lossy(response_slice).to_string()
    };
    format!("POST result: {post_result}")
}

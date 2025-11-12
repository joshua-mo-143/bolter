use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn fetch_url(ptr: *const u8, len: i32, output_ptr: *mut u8);
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
    let read = std::fs::read_dir(input.dir_path).unwrap();

    read.into_iter()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n")
}

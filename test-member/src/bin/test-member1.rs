fn main() {
    println!("Hello world from test-member1!");
}

#[unsafe(no_mangle)]
pub fn run_tool(input_ptr: *const u8, input_len: u32, out_ptr: *mut u8, output_cap: u32) -> u32 {
    let input = unsafe {
        let slice = core::slice::from_raw_parts(input_ptr, input_len as usize);
        std::str::from_utf8(slice).unwrap()
    };

    let result = format!("Hello from WASM, {}!", input);
    let result_bytes = result.as_bytes();

    let write_len = result_bytes.len().min(output_cap as usize);

    unsafe {
        core::ptr::copy_nonoverlapping(result_bytes.as_ptr(), out_ptr, write_len);
    }

    write_len as u32
}

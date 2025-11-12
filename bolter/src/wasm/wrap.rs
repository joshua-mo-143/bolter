use wasmtime::{Caller, Linker};

use crate::{
    config::Permissions,
    wasm::host_fns::{fetch_url, post_url},
};

pub fn wrap_linker(linker: &mut Linker<Permissions>) -> Result<(), Box<dyn std::error::Error>> {
    linker.func_wrap(
        "env",
        "fetch_url",
        |mut caller: Caller<'_, Permissions>, ptr: i32, len: i32, out_ptr: i32| {
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
            let resp_ptr_offset = 2000;

            let mut buf = vec![0u8; len as usize];
            memory.read(&caller, ptr as usize, &mut buf).unwrap();

            if caller.data().http() {
                let resp = fetch_url(buf).unwrap();
                let resp_bytes = resp.into_bytes();

                memory
                    .write(&mut caller, resp_ptr_offset, &resp_bytes)
                    .unwrap();

                let ptr_bytes = (resp_ptr_offset as i32).to_le_bytes();
                let len_bytes = (resp_bytes.len() as i32).to_le_bytes();

                memory
                    .write(&mut caller, out_ptr as usize, &ptr_bytes)
                    .unwrap();
                memory
                    .write(&mut caller, out_ptr as usize + 4, &len_bytes)
                    .unwrap();
            } else {
                let resp = "ERROR: No permissions to carry out a HTTP request.".to_string();
                let resp_bytes = resp.into_bytes();

                memory
                    .write(&mut caller, resp_ptr_offset, &resp_bytes)
                    .unwrap();

                let ptr_bytes = (resp_ptr_offset as i32).to_le_bytes();
                let len_bytes = (resp_bytes.len() as i32).to_le_bytes();

                memory
                    .write(&mut caller, out_ptr as usize, &ptr_bytes)
                    .unwrap();
                memory
                    .write(&mut caller, out_ptr as usize + 4, &len_bytes)
                    .unwrap();
            }
        },
    )?;

    linker.func_wrap(
        "env",
        "fetch_url_post",
        |mut caller: Caller<'_, Permissions>, ptr: i32, len: i32, out_ptr: i32| {
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();

            let mut buf = vec![0u8; len as usize];
            memory.read(&caller, ptr as usize, &mut buf).unwrap();

            let resp = post_url(buf).unwrap();
            let resp_bytes = resp.into_bytes();

            let resp_ptr_offset = 2000; // example
            memory
                .write(&mut caller, resp_ptr_offset, &resp_bytes)
                .unwrap();

            let ptr_bytes = (resp_ptr_offset as i32).to_le_bytes();
            let len_bytes = (resp_bytes.len() as i32).to_le_bytes();

            memory
                .write(&mut caller, out_ptr as usize, &ptr_bytes)
                .unwrap();
            memory
                .write(&mut caller, out_ptr as usize + 4, &len_bytes)
                .unwrap();
        },
    )?;

    Ok(())
}

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemFn, ReturnType, Type, parse_macro_input};

/// A helper macro for writing a Filigree tool in Rust.
/// This function will automatically handle pointer input/output for you so that you can focus on just writing your tool logic.
#[proc_macro_attribute]
pub fn wasi_tool(_attrs: TokenStream, body: TokenStream) -> TokenStream {
    let func = parse_macro_input!(body as ItemFn);
    let func_ident = func.sig.ident.clone();

    let inputs = func.sig.inputs.clone();

    if inputs.len() != 1 {
        panic!("There should only be one input to the `run_tool` function.");
    }

    let syn::FnArg::Typed(ty) = inputs.first().unwrap() else {
        panic!("The argument used in `run_tool` should not be self.")
    };

    let Type::Path(ref ty) = *ty.ty else {
        panic!("Type should be a path");
    };

    let input_type = if ty.path.segments.len() > 1 {
        ty.path.to_token_stream()
    } else {
        ty.path.get_ident().unwrap().into_token_stream()
    };

    let ReturnType::Type(_, ty) = func.sig.output.clone() else {
        panic!("The tool needs to return an output!");
    };

    let Type::Path(ref ty) = *ty else {
        panic!("The output type should be a path");
    };

    let output_type = if ty.path.segments.len() > 1 {
        ty.path.to_token_stream()
    } else {
        ty.path.get_ident().unwrap().into_token_stream()
    };

    let quote = quote! {
        const _: fn() = || {
            fn check_input<T: for<'a> serde::Deserialize<'a>>() {}
            fn check_output<T: serde::Serialize>() {}
            check_input::<#input_type>();
            check_output::<#output_type>();
        };

        #[unsafe(no_mangle)]
        pub fn run_tool(input_ptr: *const u8, input_len: u32, out_ptr: *mut u8, output_cap: u32) -> u32 {
            #func

            let input = unsafe {
                let slice = core::slice::from_raw_parts(input_ptr, input_len as usize);
                std::str::from_utf8(slice).unwrap()
            };
            let input = input.trim();

            let json: #input_type = serde_json::from_str(input).unwrap();
            let result = #func_ident(json);
            let json_string = serde_json::to_string(&result).unwrap();

            let result_bytes = json_string.as_bytes();
            let write_len = result_bytes.len().min(output_cap as usize);

            unsafe {
                core::ptr::copy_nonoverlapping(result_bytes.as_ptr(), out_ptr, write_len);
            }

            write_len as u32
        }
    };

    TokenStream::from(quote)
}

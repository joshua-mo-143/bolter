use wasmtime::{Engine, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

pub mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let linker_wasi = Linker::new(&engine);
    // wasmtime_wasi::p1::add_to_linker_sync(&mut linker_wasi, |ctx| ctx)?;
    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(".", "eep", DirPerms::all(), FilePerms::all())
            .unwrap()
            .build(),
    );

    let config = config::Config::from_file("test-units/test_config.json");

    for binary in config.data {
        let module = Module::from_file(&engine, &binary.path)?;
        let instance = linker_wasi.instantiate(&mut store, &module)?;

        let start: TypedFunc<(), ()> = instance.get_typed_func(&mut store, "_start").unwrap();
        start.call(&mut store, ())?;
    }

    Ok(())
}

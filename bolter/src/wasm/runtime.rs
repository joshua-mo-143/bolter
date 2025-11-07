use std::{collections::HashMap, path::Path};

use rig::completion::ToolDefinition;
use wasmtime::{Caller, Engine, Instance, Linker, Module, Store, TypedFunc};

use crate::wasm::host_fns::{fetch_url, post_url};

use super::utils::{get_tool_definition, run_wasm_tool};

pub struct WasmRuntime {
    engine: Engine,
    linker: Linker<()>,
    modules: HashMap<String, WasmModuleEntry>,
}

impl WasmRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        linker.func_wrap(
            "env",
            "fetch_url",
            |_caller: Caller<'_, ()>, ptr: i32, len: i32| {
                let _ = fetch_url(ptr, len);

                return 0;
            },
        )?;

        linker.func_wrap(
            "env",
            "fetch_url_post",
            |mut caller: Caller<'_, ()>, ptr: i32, len: i32, out_ptr: i32| {
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

        Ok(Self {
            engine,
            linker,
            modules: HashMap::new(),
        })
    }

    pub fn with_modules_from_file<P>(path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
    {
        let runtime = Self::new()?;

        runtime.add_modules_from_file(path)
    }

    pub fn get_tool(&mut self, module_name: &str) -> Option<&mut WasmModuleEntry> {
        self.modules.get_mut(module_name)
    }

    pub fn add_modules_from_file<P>(mut self, path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
    {
        let config = crate::config::Config::from_file(path);

        for binary in config.data {
            self.add_module(binary)?;
        }

        Ok(self)
    }

    pub fn add_module(
        &mut self,
        cfg: crate::config::Module,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let module = Module::from_file(&self.engine, &cfg.path)?;
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, &module)?;

        let tooldef = get_tool_definition(&instance, &mut store)?;
        let tooldef = ToolDefinition {
            name: cfg.title.clone(),
            description: cfg.description,
            parameters: serde_json::from_str(&tooldef).unwrap(),
        };
        let func: TypedFunc<(i32, i32, i32, i32), i32> =
            instance.get_typed_func(&mut store, "run_tool")?;

        let entry = WasmModuleEntry::new(store, tooldef, module, instance, func);

        self.modules.insert(cfg.title, entry);

        Ok(())
    }

    pub fn get_tooldefs(&self) -> Vec<ToolDefinition> {
        self.modules.values().map(|v| v.tooldef.clone()).collect()
    }

    pub fn run_tool(
        &mut self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let Some(entry) = self.get_tool(name) else {
            return Err("Could not find tool".into());
        };

        run_wasm_tool(&entry.instance, &mut entry.store, &entry.func, args)
    }
}

pub struct WasmModuleEntry {
    pub store: Store<()>,
    pub tooldef: ToolDefinition,
    pub module: Module,
    pub instance: Instance,
    pub func: TypedFunc<(i32, i32, i32, i32), i32>,
}

impl WasmModuleEntry {
    pub fn new(
        store: Store<()>,
        tooldef: ToolDefinition,
        module: Module,
        instance: Instance,
        func: TypedFunc<(i32, i32, i32, i32), i32>,
    ) -> Self {
        Self {
            store,
            tooldef,
            module,
            instance,
            func,
        }
    }
}

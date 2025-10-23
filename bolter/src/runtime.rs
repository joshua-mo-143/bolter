use std::collections::HashMap;

use rig::completion::ToolDefinition;
use wasmtime::{Engine, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, p1::WasiP1Ctx};

use crate::{get_tool_definition, run_wasm_tool};

pub struct WasiRuntime {
    engine: Engine,
    linker: Linker<WasiP1Ctx>,
    modules: HashMap<String, WasiModuleEntry>,
}

impl WasiRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |ctx| ctx)?;

        Ok(Self {
            engine,
            linker,
            modules: HashMap::new(),
        })
    }

    pub fn get_tool(&mut self, module_name: &str) -> Option<&mut WasiModuleEntry> {
        self.modules.get_mut(module_name)
    }

    pub fn add_module(
        &mut self,
        cfg: crate::config::Module,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let module = Module::from_file(&self.engine, &cfg.path)?;
        let mut store = Store::new(
            &self.engine,
            WasiCtxBuilder::new()
                .inherit_stdio()
                .preopened_dir(".", ".", DirPerms::all(), FilePerms::all())
                .unwrap()
                .build_p1(),
        );
        let instance = self.linker.instantiate(&mut store, &module)?;

        let tooldef = get_tool_definition(&instance, &mut store)?;
        let tooldef = ToolDefinition {
            name: cfg.title.clone(),
            description: cfg.description,
            parameters: serde_json::from_str(&tooldef).unwrap(),
        };
        let func: TypedFunc<(i32, i32, i32, i32), i32> =
            instance.get_typed_func(&mut store, "run_tool")?;

        let entry = WasiModuleEntry::new(store, tooldef, module, instance, func);

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

pub struct WasiModuleEntry {
    pub store: Store<WasiP1Ctx>,
    pub tooldef: ToolDefinition,
    pub module: Module,
    pub instance: Instance,
    pub func: TypedFunc<(i32, i32, i32, i32), i32>,
}

impl WasiModuleEntry {
    pub fn new(
        store: Store<WasiP1Ctx>,
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

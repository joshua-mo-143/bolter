use std::{collections::HashMap, path::Path};

use rig::completion::ToolDefinition;
use wasmtime::{Engine, Instance, Linker, Module, Store, TypedFunc};

use crate::{config::Permissions, wasm::wrap::wrap_linker};

use super::utils::{get_tool_definition, run_wasm_tool};

pub struct WasmRuntime {
    engine: Engine,
    linker: Linker<Permissions>,
    modules: HashMap<String, WasmModuleEntry>,
}

impl WasmRuntime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        wrap_linker(&mut linker)?;

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
        let mut store = Store::new(&self.engine, cfg.permissions);
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
    pub store: Store<Permissions>,
    pub tooldef: ToolDefinition,
    pub module: Module,
    pub instance: Instance,
    pub func: TypedFunc<(i32, i32, i32, i32), i32>,
}

impl WasmModuleEntry {
    pub fn new(
        store: Store<Permissions>,
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

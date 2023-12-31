use std::fs::File;
use std::io::Read;
use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Linker, Component};
use crate::example::grep::types::LineList;
use crate::Guest;

pub struct GrepContext {
    store: Store<()>,
    filter: Guest,
}

impl GrepContext {
    pub fn new(path_to_filter: &Option<String>) -> anyhow::Result<GrepContext> {
        let engine = Engine::new(&Self::create_config())?;
        let mut store = Store::new(&engine, ());
        let linker = Linker::new(&engine);
        let binary = Self::read_filter(path_to_filter)?;
        let component = Component::from_binary(&engine, &binary)?;
        let (filter, _) = Guest::instantiate(&mut store, &component, &linker)?;
        Ok(
            GrepContext {
                store,
                filter,
            }
        )
    }

    fn create_config() -> Config {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config
    }

    fn read_filter(path_to_filter: &Option<String>) -> anyhow::Result<Vec<u8>> {
        let buffer = if let Some(path) = path_to_filter {
            let mut buffer = vec![];
            let mut file = File::open(path)?;
            file.read_to_end(&mut buffer)?;
            buffer
        } else {
            include_bytes!("./wasm/filter_id.wasm").to_vec()
        };
        Ok(buffer)
    }

    pub fn apply_filter(&mut self, input: &LineList) -> Option<LineList> {
        self.filter.interface0.call_apply(&mut self.store, input).unwrap().or(None)
    }
}
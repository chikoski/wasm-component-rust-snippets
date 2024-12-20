use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Component, Linker};
use std::path::Path;
use wasmtime_wasi::bindings::Command;
use crate::host::Host;

pub struct Runtime {
    engine: Engine,
    linker: Linker<Host>,
}

impl Runtime {
    pub fn store(&self) -> Store<Host> {
        let host = Host::default();
        Store::new(&self.engine, host)
    }
    pub fn load_component(&self, filename: impl AsRef<Path>) -> anyhow::Result<Component> {
        Component::from_file(&self.engine, filename)
    }

    pub async fn instantiate_command(&self, store: &mut Store<Host>, component: &Component) -> anyhow::Result<Command> {
        Command::instantiate_async(store, &component, &self.linker).await
    }

    pub fn enable_wasi(&mut self) -> anyhow::Result<()> {
        wasmtime_wasi::add_to_linker_async(&mut self.linker)
    }
}

impl TryFrom<&Config> for Runtime {
    type Error = wasmtime::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let engine = Engine::new(config)?;
        let linker = Linker::new(&engine);
        let runtime = Runtime { engine, linker };
        Ok(runtime)
    }
}
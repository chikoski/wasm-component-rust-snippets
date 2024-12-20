use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Component, Linker};
use std::path::Path;
use wasmtime_wasi::bindings::Command;
use wasmtime_wasi::bindings::exports::wasi::cli::run::Guest;
use crate::host::Host;

pub struct Runtime {
    engine: Engine,
    linker: Linker<Host>,
    store: Store<Host>,
}

impl Runtime {

    pub fn load_component(&self, filename: impl AsRef<Path>) -> anyhow::Result<Component> {
        Component::from_file(&self.engine, filename)
    }

    pub async fn instantiate_command(&mut self, component: &Component) -> anyhow::Result<Command> {
        let linker = &self.linker;
        let store = &mut self.store;
        Command::instantiate_async(store, &component, linker).await
    }

    pub async fn call_run(&mut self, run: &Guest) -> anyhow::Result<()> {
        let store = &mut self.store;
        let _ = run.call_run(store).await?;
        Ok(())
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
        let store = Store::new(&engine, Host::default());
        let runtime = Runtime { engine, linker, store };
        Ok(runtime)
    }
}
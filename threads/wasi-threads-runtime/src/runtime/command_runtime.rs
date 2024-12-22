use std::path::Path;
use wasmtime::component::{Component, Linker};
use wasmtime_wasi::bindings::Command;
use wasmtime_wasi::bindings::exports::wasi::cli::run::Guest;
use wasmtime::{Config, Engine, Store};
use crate::cli::Cli;
use crate::runtime::preview2_host::Preview2Host;

pub struct CommandRuntime {
    engine: Engine,
    linker: Linker<Preview2Host>,
    store: Store<Preview2Host>,
}

impl CommandRuntime {
    pub async fn try_run(cli: &Cli) -> anyhow::Result<()> {
        let mut config = Config::default();
        config.async_support(true);
        let mut runtime = CommandRuntime::try_from(&config)?;
        runtime.enable_wasi()?;

        let component = runtime.load_component(&cli.filename)?;
        let command = runtime.instantiate_command(&component).await?;
        let run = command.wasi_cli_run();
        runtime.call_run(run).await
    }

    fn load_component(&self, filename: impl AsRef<Path>) -> anyhow::Result<Component> {
        Component::from_file(&self.engine, filename)
    }

    async fn instantiate_command(&mut self, component: &Component) -> anyhow::Result<Command> {
        let linker = &self.linker;
        let store = &mut self.store;
        Command::instantiate_async(store, &component, linker).await
    }

    async fn call_run(&mut self, run: &Guest) -> anyhow::Result<()> {
        let store = &mut self.store;
        let _ = run.call_run(store).await?;
        Ok(())
    }

    fn enable_wasi(&mut self) -> anyhow::Result<()> {
        wasmtime_wasi::add_to_linker_async(&mut self.linker)
    }
}

impl TryFrom<&Config> for CommandRuntime {
    type Error = wasmtime::Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let engine = Engine::new(config)?;
        let linker = Linker::new(&engine);
        let store = Store::new(&engine, Preview2Host::default());
        let runtime = CommandRuntime { engine, linker, store };
        Ok(runtime)
    }
}


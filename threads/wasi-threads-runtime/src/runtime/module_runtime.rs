use crate::runtime::preview1_host::Preview1Host;
use std::path::Path;
use wasmtime::{Config, Engine, Instance, Linker, MemoryType, Module, SharedMemory, Store};
use wasmtime_wasi::preview1;
use crate::cli::Cli;

pub struct ModuleRuntime {
    engine: Engine,
    linker: Linker<Preview1Host>,
    store: Store<Preview1Host>,
}

impl TryFrom<&Config> for ModuleRuntime {
    type Error = anyhow::Error;
    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let engine = Engine::new(config)?;
        let linker = Linker::new(&engine);
        let store = Store::new(&engine, Preview1Host::default());
        Ok(ModuleRuntime { engine, linker, store })
    }
}

impl ModuleRuntime {
    pub async fn try_run(cli: &Cli) -> anyhow::Result<()> {
        let mut config = Config::default();
        config.async_support(true);

        let mut runtime = ModuleRuntime::try_from(&config)?;
        runtime.enable_wasi()?;
        let module = runtime.load_module(&cli.filename)?;
        let instance = runtime.instantiate(&module).await?;
        runtime.call_start(&instance).await
    }

    fn load_module(&self, filename: impl AsRef<Path>) -> anyhow::Result<Module> {
        Module::from_file(&self.engine, filename)
    }

    async fn instantiate(&mut self, module: &Module) -> anyhow::Result<Instance> {
        let linker = &mut self.linker;
        let memory = SharedMemory::new(&self.engine, MemoryType::shared(17, 17))?;
        linker.define(&mut self.store, "env", "memory", memory)?;
        linker.instantiate_async(&mut self.store, module).await
    }

    fn enable_wasi(&mut self) -> anyhow::Result<()> {
        preview1::add_to_linker_async(&mut self.linker, |t| t)
    }

    async fn call_start(&mut self, instance: &Instance) -> anyhow::Result<()> {
        let store = &mut self.store;
        let start = instance.get_typed_func::<(), ()>(store, "_start")?;

        let store = &mut self.store;
        start.call_async(store, ()).await
    }
}

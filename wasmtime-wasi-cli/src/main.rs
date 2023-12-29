use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Component, Linker};
use wasmtime_wasi::preview2::command::Command;
use crate::wasi_view_impl::WasiViewImpl;

mod wasi_view_impl;

#[async_std::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", e);
    }
}

async fn run() -> anyhow::Result<()>{
    let engine = Engine::new(&create_config())?;
    let wasi_view = WasiViewImpl::new();
    let mut store = Store::new(&engine, wasi_view);
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::preview2::command::add_to_linker(&mut linker)?;

    let component = load_component(&engine)?;
    let instance = linker.instantiate_async(&mut store, &component).await?;
    let command = Command::new(&mut store, &instance)?;

    let run = command.wasi_cli_run();
    let _ = run.call_run(&mut store).await?;
    Ok(())
}

fn create_config() -> Config {
    let mut config = Config::new();
    //config.async_support(true);
    config.wasm_component_model(true);
    config
}

fn load_component(engine: &Engine) -> anyhow::Result<Component> {
    let binary = include_bytes!("./wasm/hello-world.wasm").to_vec();
    Component::from_binary(&engine, &binary)
}

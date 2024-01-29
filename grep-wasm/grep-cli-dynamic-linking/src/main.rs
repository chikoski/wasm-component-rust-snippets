use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{Component, Linker};
use wasmtime_wasi::preview2::command;

use app_config::AppConfig;
use args::Args;

use crate::wasi_view_impl::WasiViewImpl;

mod app_config;
mod app_error;
mod args;
mod wasi_view_impl;

#[async_std::main]
async fn main() {
    let args = Args::parse();
    let config: AppConfig = args.into();
    if let Err(e) = start(config).await {
        println!("{}", e);
    }
}

async fn start(config: AppConfig) -> anyhow::Result<()> {
    let engine = Engine::new(&config.wasmtime_config)?;
    let wasi_view = WasiViewImpl::new(&config);
    let mut store = Store::new(&engine, wasi_view);
    let mut linker = Linker::new(&engine);

    command::add_to_linker(&mut linker)?;
    let filter_component = Component::from_binary(&engine, &config.filter)?;
    let host_component = Component::from_binary(&engine, &config.host_code)?;

    linker.instantiate_async(&mut store, &filter_component).await?;
    let host = linker.instantiate_async(&mut store, &host_component).await?;
    let cmd = command::Command::new(&mut store, &host)?;
    cmd.wasi_cli_run();
    Ok(())
}

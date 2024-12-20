use crate::cli::Cli;
use clap::Parser;
use runtime::Runtime;
use wasmtime::Config;
use wasmtime_wasi::WasiView;

mod cli;
mod host;
mod runtime;

#[tokio::main]
async fn main() {
    match start().await {
        Ok(_) => (),
        Err(e) => println!("{e:?}"),
    }
}

async fn start() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut config = Config::default();
    config.async_support(true);
    let mut runtime = Runtime::try_from(&config)?;
    let mut store = runtime.store();
    runtime.enable_wasi()?;

    let component = runtime.load_component(&cli.filename)?;
    let command = runtime.instantiate_command(&mut store, &component).await?;
    let run = command.wasi_cli_run();
    let _ = run.call_run(&mut store).await?;
    Ok(())
}
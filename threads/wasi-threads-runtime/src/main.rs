use crate::cli::Cli;
use clap::Parser;
use runtime::Runtime;
use wasmtime::Config;

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
    runtime.enable_wasi()?;

    let component = runtime.load_component(&cli.filename)?;
    let command = runtime.instantiate_command(&component).await?;
    let run = command.wasi_cli_run();
    runtime.call_run(run).await
}
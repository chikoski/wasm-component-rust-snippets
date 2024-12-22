use crate::cli::Cli;
use crate::runtime::ModuleRuntime;
use clap::Parser;
use runtime::CommandRuntime;
use wasmtime::Config;

mod cli;
mod runtime;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match try_run(&cli).await {
        Ok(_) => (),
        Err(e) => println!("{e:?}"),
    }
}

async fn try_run(cli: &Cli) -> anyhow::Result<()> {
    let r = try_as_component(&cli).await;
    if r.is_err() {
        try_as_module(&cli).await
    } else {
        Ok(())
    }
}

async fn try_as_component(cli: &Cli) -> anyhow::Result<()> {
    let mut config = Config::default();
    config.async_support(true);
    let mut runtime = CommandRuntime::try_from(&config)?;
    runtime.enable_wasi()?;

    let component = runtime.load_component(&cli.filename)?;
    let command = runtime.instantiate_command(&component).await?;
    let run = command.wasi_cli_run();
    runtime.call_run(run).await
}

async fn try_as_module(cli: &Cli) -> anyhow::Result<()> {
    let mut config = Config::default();
    config.async_support(true);

    let mut runtime = ModuleRuntime::try_from(&config)?;
    runtime.enable_wasi()?;
    let module = runtime.load_module(&cli.filename)?;
    let instance = runtime.instantiate(&module).await?;
    runtime.call_start(&instance).await
}
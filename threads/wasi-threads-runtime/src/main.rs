use crate::cli::Cli;
use crate::runtime::ModuleRuntime;
use clap::Parser;
use runtime::CommandRuntime;
use futures_util::TryFutureExt;

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
    CommandRuntime::try_run(&cli)
        .or_else(|_| async move {
            ModuleRuntime::try_run(&cli).await
        }).await
}
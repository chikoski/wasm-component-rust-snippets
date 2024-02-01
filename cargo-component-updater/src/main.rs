use clap::Parser;
use cli::Cli;
use context::Context;
use env_logger::Env;

mod cli;
mod context;

fn main() -> anyhow::Result<()> {
    let env = Env::new().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    let context: Context = Cli::parse().into();
    context.update_rs()?;
    context.update_manifest()?;
    Ok(())
}

use crate::context::Context;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    root: Option<String>,
}

impl From<Cli> for Context {
    fn from(value: Cli) -> Self {
        let root = value.root.unwrap_or(".".to_string());
        Context { root }
    }
}

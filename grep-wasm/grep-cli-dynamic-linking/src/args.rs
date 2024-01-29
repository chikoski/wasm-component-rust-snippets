use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub filter: Option<String>,
    pub files: Vec<String>,
}

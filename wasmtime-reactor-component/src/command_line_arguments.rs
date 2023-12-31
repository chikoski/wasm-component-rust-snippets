use clap::Parser;

#[derive(Parser, Debug)]
pub struct CommandLineArguments {
    #[arg(short, long)]
    pub filter_file: Option<String>,
    pub file_list: Vec<String>
}
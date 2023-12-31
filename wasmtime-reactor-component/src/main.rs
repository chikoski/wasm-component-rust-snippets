use std::fs::read_to_string;

use clap::Parser;
use wasmtime::component::bindgen;

use crate::command_line_arguments::CommandLineArguments;
use crate::example::grep::types::{Line, LineList};
use crate::grep_context::GrepContext;

mod command_line_arguments;
mod grep_context;

bindgen!();

fn main() {
    let arguments = CommandLineArguments::parse();
    if let Err(e) = run(arguments) {
        println!("{}", e);
    }
}

fn run(arguments: CommandLineArguments) -> anyhow::Result<()> {
    let mut context = GrepContext::new(&arguments.filter_file)?;
    for file in arguments.file_list {
        if let Ok(line_list) = read_as_line_list(&file) {
            if let Some(found_lines) = context.apply_filter(&line_list) {
                print_matched_lines(&file, &found_lines);
            }
        }
    }
    Ok(())
}

fn read_as_line_list(path: &String) -> anyhow::Result<LineList> {
    let lines = read_to_string(path)?.lines().enumerate().map(|(index, line)| {
        Line {
            line_number: index as u32 + 1,
            text: line.to_string(),
        }
    }).collect::<Vec<Line>>();
    Ok(LineList { lines })
}

fn print_matched_lines(path: &str, lines_list: &LineList) {
    println!("{}:", path);
    for line in lines_list.lines {
        println!("{:3}: {}", line.line_number, line.text);
    }
}
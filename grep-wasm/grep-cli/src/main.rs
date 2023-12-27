use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Context;

use bindings::example::grep::file_as_multiline::grep;
use bindings::example::grep::types::{Line, LineList};

cargo_component_bindings::generate!();

fn main() {
    let target = PathBuf::from("./src/main.rs");
    if let Err(e) = run(target) {
        println!("{}", e);
    }
}

fn run(path: PathBuf) -> anyhow::Result<()> {
    let buffer = read_to_string(path).context("Fail on read_to_string")?;
    let lines: Vec<Line> = buffer
        .lines()
        .enumerate()
        .map(|(index, value)| new_line(index, value))
        .collect();
    let lines = lines.into();
    if let Some(lines) = grep(&lines) {
        for line in lines.lines {
            println!("{}", line);
        }
    }
    Ok(())
}

fn new_line(index: usize, line: &str) -> Line {
    Line {
        line_number: index as u32,
        text: line.to_owned(),
    }
}

impl Into<LineList> for Vec<Line> {
    fn into(self) -> LineList {
        LineList { lines: self }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3}: {}", self.line_number, self.text)
    }
}

use regex::Regex;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const CARGO_COMPONENT_BINDINGS: &'static str = r"cargo_component_bindings::generate\!\(\);";
const WIT_BINDGEN: &'static str = "mod bindings;";

pub fn udpate_code(path: &PathBuf) -> anyhow::Result<()> {
    let regex = Regex::new(CARGO_COMPONENT_BINDINGS)?;

    let mut lines = read_to_string(path)?
        .lines()
        .into_iter()
        .map(|line| {
            let line = regex.replace(line, WIT_BINDGEN);
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    lines.push('\n');
    let mut file = File::create(path)?;
    file.write_all(lines.as_bytes())?;
    Ok(())
}

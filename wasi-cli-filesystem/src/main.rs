use std::{env, fs, fs::read_to_string, path::PathBuf};

mod bindings;

fn main() {
    println!("Hello, world!");
    if let Err(e) = print_cwd() {
        println!("{}", e);
    }
    if let Err(e) = print_entries_in_root() {
        println!("{}", e);
    }
    if let Err(e) = print_itself() {
        println!("{}", e);
    }
}

fn print_cwd() -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    println!("cwd = {}", cwd.display());
    Ok(())
}

fn print_entries_in_root() -> anyhow::Result<()> {
    for path in read_dir(".")? {
        println!("{}", path.display())
    }
    Ok(())
}

fn read_dir(path: &str) -> anyhow::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let entries = entries
        .filter_map(|f| {
            if let Ok(entry) = f {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();
    Ok(entries)
}

fn print_itself() -> anyhow::Result<()> {
    let itself = "./src/main.rs";
    let content = read_to_string(itself)?;
    println!("{}", content);
    Ok(())
}

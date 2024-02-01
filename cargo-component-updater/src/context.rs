use log::{info, warn};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::PathBuf;

use replace_dependencies::replace_dependency;

mod error;
mod replace_dependencies;
mod update_code;

pub struct Context {
    pub root: String,
}

impl Context {
    pub fn update_rs(&self) -> anyhow::Result<Vec<PathBuf>> {
        traverse_and_apply("**/{main,lib}.rs", &self.root, update_code::udpate_code)
    }
    pub fn update_manifest(&self) -> anyhow::Result<Vec<PathBuf>> {
        traverse_and_apply("**/Cargo.toml", &self.root, replace_dependency)
    }
}

fn traverse_and_apply<F>(glob: &str, root: &str, updator: F) -> anyhow::Result<Vec<PathBuf>>
where
    F: (Fn(&PathBuf) -> anyhow::Result<()>) + Sync,
{
    globmatch::Builder::new(glob)
        .build(root)
        .into_par_iter()
        .for_each(|matcher| {
            for m in matcher.into_iter() {
                if let Ok(path) = m {
                    info!("Start updating: {}", path.to_string_lossy());
                    if let Err(e) = updator(&path) {
                        warn!("{}", e);
                    }
                }
            }
        });
    Ok(vec![])
}

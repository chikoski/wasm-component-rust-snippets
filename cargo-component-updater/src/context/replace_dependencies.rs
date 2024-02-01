use super::error::UpdateError;
use anyhow::Ok;
use rayon::str;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{ffi::OsStr, fs::read_to_string};
use toml::{Table, Value};

const CARGO_COMPONENT_BINDINGS: &'static str = "cargo-component-bindings";
const WIT_BINDGEN: &'static str = "wit-bindgen";

pub fn replace_dependency(path: &PathBuf) -> anyhow::Result<()> {
    if let Result::Ok(dependencies) = Dependencies::try_from(path) {
        if dependencies.has_cargo_component_bindings {
            let package_dir = path.parent().ok_or(UpdateError::NoPackageDirFound)?;
            let cargo = CargoCommand::new(package_dir);
            cargo.remove(CARGO_COMPONENT_BINDINGS)?;
            if !dependencies.has_wit_bindgen {
                cargo.add(WIT_BINDGEN)?;
            }
        }
    }
    Ok(())
}

struct Dependencies {
    has_wit_bindgen: bool,
    has_cargo_component_bindings: bool,
}

impl Dependencies {
    fn new(has_wit_bindgen: bool, has_cargo_component_bindings: bool) -> Dependencies {
        Dependencies {
            has_wit_bindgen,
            has_cargo_component_bindings,
        }
    }
}

impl TryFrom<&PathBuf> for Dependencies {
    type Error = anyhow::Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let toml = read_to_string(value)?;
        let toml = toml.parse::<Table>()?;
        let dependencies = toml
            .get("dependencies")
            .ok_or(UpdateError::NoDependenciesTable)?;
        if let Value::Table(dependencies) = dependencies {
            let has_wit_bindgen = dependencies.contains_key(WIT_BINDGEN);
            let has_cargo_component_bindings = dependencies.contains_key(CARGO_COMPONENT_BINDINGS);
            Ok(Dependencies::new(
                has_wit_bindgen,
                has_cargo_component_bindings,
            ))
        } else {
            Ok(Dependencies::new(false, false))
        }
    }
}

struct CargoCommand<'a> {
    package_dir: &'a Path,
}

impl<'a> CargoCommand<'a> {
    fn new(package_dir: &Path) -> CargoCommand {
        CargoCommand { package_dir }
    }

    fn add(&self, package: &str) -> anyhow::Result<()> {
        let args = ["add", package];
        self.execute(&args)
    }

    fn remove(&self, package: &str) -> anyhow::Result<()> {
        let args = ["remove", package];
        self.execute(&args)
    }

    fn execute<I, S>(&self, arguments: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let _ = Command::new("cargo")
            .args(arguments)
            .current_dir(self.package_dir)
            .output()?;
        Ok(())
    }
}

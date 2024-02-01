use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Cargo.toml has no dependencies table")]
    NoDependenciesTable,
    #[error("No package dir is found")]
    NoPackageDirFound,
}

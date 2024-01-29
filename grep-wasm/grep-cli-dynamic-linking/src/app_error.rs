use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No filter file is specified.")]
    NoFilterIsSpecified
}
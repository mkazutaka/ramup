use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("FailedCommand: {0}")]
    CommandError(String),
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        unimplemented!()
    }
}

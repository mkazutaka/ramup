use fs_extra;
use plist;
use std;
use std::path::StripPrefixError;
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

impl From<plist::Error> for AppError {
    fn from(_: plist::Error) -> Self {
        unimplemented!()
    }
}

impl From<std::path::StripPrefixError> for AppError {
    fn from(_: std::path::StripPrefixError) -> Self {
        unimplemented!()
    }
}

impl From<fs_extra::error::Error> for AppError {
    fn from(_: fs_extra::error::Error) -> Self {
        unimplemented!()
    }
}

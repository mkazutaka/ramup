use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileProgressError {
    #[error(transparent)]
    FsExtraError(#[from] fs_extra::error::Error),
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("File doesn't exist: {0}")]
    NotExist(String),

    #[error("Failed to get metadata: {0}")]
    FailedToGetMetaData(String),

    #[error("File is not symbolic link: {0}")]
    NotSymbolicLink(String),
}

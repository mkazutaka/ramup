use crate::apperror::FileSystemError;
use crate::appfs;
use crate::apppath::AbsPath;
use anyhow::{Context, Result};

pub struct Restore {}

impl Restore {
    pub fn restore(from: &AbsPath, to: &AbsPath) -> Result<String> {
        Restore::validate(from, to)?;
        Restore::_restore(from, to)
    }

    fn validate(_from: &AbsPath, to: &AbsPath) -> Result<()> {
        if !to.as_ref().exists() {
            return Err(anyhow::anyhow!(FileSystemError::NotExist(to.to_string())));
        }

        let to_meta = std::fs::symlink_metadata(&to)
            .with_context(|| FileSystemError::FailedToGetMetaData(to.to_string()))?;

        if !to_meta.file_type().is_symlink() {
            return Err(anyhow::anyhow!(FileSystemError::NotSymbolicLink(
                to.to_string()
            )));
        }

        Ok(())
    }

    fn _restore(from: &AbsPath, to: &AbsPath) -> Result<String> {
        std::fs::remove_file(to).with_context(|| "Cannot Delete file")?;
        appfs::relocate(&from, &to).with_context(|| "cannot relocate file")?;
        Ok(from.to_string())
    }
}

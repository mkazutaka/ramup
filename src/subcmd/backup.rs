use crate::apperror::FileSystemError;
use crate::appfs;
use crate::apppath::AbsPath;
use anyhow::{Context, Result};

pub struct Backup {}

impl Backup {
    pub fn backup(from: &AbsPath, to: &AbsPath) -> Result<String> {
        Backup::validate(from, to)?;
        Backup::_backup(from, to)
    }

    fn validate(from: &AbsPath, _to: &AbsPath) -> Result<()> {
        if !&from.as_ref().exists() {
            return Err(anyhow::anyhow!(FileSystemError::NotExist(from.to_string())));
        };

        let from_meta = std::fs::symlink_metadata(&from)
            .with_context(|| FileSystemError::FailedToGetMetaData(from.to_string()))?;
        if from_meta.file_type().is_symlink() {
            return Err(anyhow::anyhow!(FileSystemError::FileIsAlreadySymbolicLink(
                from.to_string()
            )));
        }

        Ok(())
    }

    fn _backup(from: &AbsPath, to: &AbsPath) -> Result<String> {
        std::fs::create_dir_all(&to.parent()?)?;
        appfs::relocate(&from, &to)?;
        std::os::unix::fs::symlink(&to, &from)?;
        Ok(from.to_string())
    }
}

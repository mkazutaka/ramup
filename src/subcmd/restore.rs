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
        let to_meta = std::fs::symlink_metadata(&to)?;
        if !to_meta.file_type().is_symlink() {
            return Err(anyhow::anyhow!("{} is not symlink", to.to_string()));
        }
        Ok(())
    }

    fn _restore(from: &AbsPath, to: &AbsPath) -> Result<String> {
        std::fs::remove_file(to).with_context(|| "Cannot Delete file")?;
        appfs::relocate(&from, &to)?;
        Ok(from.to_string())
    }
}

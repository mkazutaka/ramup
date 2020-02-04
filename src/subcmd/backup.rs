use crate::appfs;
use crate::apppath::AbsPath;
use anyhow::Result;

pub struct Backup {}

impl Backup {
    pub fn backup(from: &AbsPath, to: &AbsPath) -> Result<String> {
        Backup::validate(from, to)?;
        Backup::_backup(from, to)
    }

    fn validate(from: &AbsPath, to: &AbsPath) -> Result<()> {
        if !&from.as_ref().exists() {
            println!("skip: {:?}", from);
            return Ok(());
        } else if to.as_ref().exists() {
            println!("skip: {:?}", to);
            return Ok(());
        } else {
            println!("start: {:?}", to)
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

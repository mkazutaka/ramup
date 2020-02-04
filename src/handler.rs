use crate::appenv;
use crate::apppath::AbsPath;
use crate::maccmd::{DiskUtil, HdiUtil};
use crate::ram::RAM;
use crate::state::State;
use crate::subcmd::{Backup, Restore};
use anyhow::{Context, Result};
use std::convert::TryFrom;
use std::path::Path;

pub struct Handler {
    ram: RAM,
    state: State,
}

impl Handler {
    pub fn new(ram: RAM, state: State) -> Self {
        Handler { ram, state }
    }

    pub fn backup(&mut self, sources: Vec<String>) -> Result<()> {
        Handler::mount(&self.ram)?;

        let target_base_path = AbsPath::try_from(&self.ram.mount_path)?.join(&self.ram.name)?;
        for source in &sources {
            let source = AbsPath::new(&source)?;
            let target = target_base_path.join(&source)?;

            let path = Backup::backup(&source, &target);
            match path {
                Ok(path) => self.state.add(path),
                Err(err) => {
                    if err.downcast_ref::<fs_extra::error::Error>().is_some() {
                        println!("restore: {:?}", source.as_ref());
                        self.restore(vec![source.to_string()])
                            .with_context(|| "Failed to restore")
                            .unwrap();
                    }
                    Err(err)
                }
            }?;
        }
        Ok(())
    }

    pub fn restore(&mut self, targets: Vec<String>) -> Result<()> {
        let source_base_path = AbsPath::try_from(&self.ram.mount_path)?.join(&self.ram.name)?;

        for target in &targets {
            let source = source_base_path.join(&target)?;
            let target = AbsPath::new(&target)?;

            Restore::restore(&source, &target)?;
            self.state.remove(target)?;
        }
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        let sp = appenv::state();
        if Path::new(&sp).exists() {
            std::fs::remove_file(&sp).with_context(|| "Failed to delete state file")?;
        }
        Handler::unmount(&self.ram)
    }

    fn mount(ram: &RAM) -> Result<()> {
        if HdiUtil::exist_volume(&ram.name)? {
            return Ok(());
        }
        let mountpoint = HdiUtil::attach(ram.size)?;
        DiskUtil::erasevolume(&ram.name, &mountpoint)
    }

    fn unmount(ram: &RAM) -> Result<()> {
        if !HdiUtil::exist_volume(&ram.name)? {
            return Ok(());
        }
        HdiUtil::detach_volume(&ram.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempdir::TempDir;

    macro_rules! check {
        ($e:expr) => {
            match $e {
                Ok(t) => t,
                Err(e) => panic!("{} failed with: {}", stringify!($e), e),
            }
        };
    }

    #[test]
    #[serial]
    #[cfg(target_os = "macos")]
    fn backup_and_restore() {
        let mount_tmp_dir = check!(TempDir::new("ramup-source"));
        let mount_path = mount_tmp_dir.path();
        let mount_str = mount_path.to_str().unwrap();

        let target_tmp_dir = check!(TempDir::new("ramup-target"));
        let target_path = target_tmp_dir.path();
        let target_str = target_path.to_str().unwrap();

        let dir = TempDir::new("ramup-config").unwrap();
        let path = dir.path().join("state.toml").to_string_lossy().to_string();
        std::env::set_var(crate::appenv::KEY_STATE_PATH, path);

        let toml = format!(
            r#"
                 [ram]
                 devname = "RAMDisk"
                 size = 8388607
                 mount_path = "{}"
            "#,
            mount_str
        );
        let ram = RAM::new_from_str(&toml).unwrap();
        let state = State::load();

        // Backup
        let mut handler = Handler::new(ram, state);
        check!(handler.backup(vec![target_str.to_string()]));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), true);
        assert_eq!(m.file_type().is_dir(), false);

        // Is Correct SymLink
        //        let sym_file_path = mount_path
        //            .join(&ram.name)
        //            .join(target_path.strip_prefix("/").unwrap());
        //        assert_eq!(sym_file_path, check!(fs::read_link(target_str)));

        // Restore
        check!(handler.restore(vec![target_str.to_string()]));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), false);
        assert_eq!(m.file_type().is_dir(), true);

        let ram = RAM::new_from_str(&toml).unwrap();
        check!(Handler::unmount(&ram));
    }
}

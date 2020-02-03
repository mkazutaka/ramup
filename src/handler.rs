use crate::application::Application;
use crate::env;
use crate::maccmd::{DiskUtil, HdiUtil};
use crate::path::AbsPath;
use crate::ram::RAM;
use crate::state::State;
use anyhow::{Context, Result};
use fs_extra::dir::CopyOptions;
use std::convert::TryFrom;
use std::path::Path;

pub struct Handler {
    ram: RAM,
    apps: Vec<Application>,
    state: State,
}

impl Handler {
    pub fn new(ram: RAM, apps: &[Application], state: State) -> Self {
        let apps = apps.to_vec();
        Handler { ram, apps, state }
    }

    pub fn backup_all(&mut self) -> Result<()> {
        Handler::mount(&self.ram)?;
        Handler::_backup_all(&self.ram, &self.apps, &mut self.state)
    }

    fn _backup_all(ram: &RAM, apps: &[Application], state: &mut State) -> Result<()> {
        for app in apps {
            for path in &app.paths {
                Handler::_backup(path, ram, state)?
            }
        }
        Ok(())
    }

    pub fn backup<P: AsRef<Path>>(&mut self, s_path: P) -> Result<()> {
        Handler::mount(&self.ram)?;
        Handler::_backup(s_path, &self.ram, &mut self.state)
    }

    fn _backup<P: AsRef<Path>>(s_path: P, ram: &RAM, state: &mut State) -> Result<()> {
        let t_path = AbsPath::try_from(&ram.mount_path)?
            .join(&ram.name)?
            .join(&s_path)?;
        let t_dir = t_path.parent()?;

        std::fs::create_dir_all(&t_dir)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(&s_path, &t_dir, &option).with_context(|| "Failed moving files")?;
        std::os::unix::fs::symlink(&t_path, &s_path)?;

        state.add_and_save(&s_path)?;
        Ok(())
    }

    pub fn restore_all(&mut self) -> Result<()> {
        Handler::_restore_all(&self.ram, &self.apps, &mut self.state)?;
        self.clean()
    }

    fn _restore_all(ram: &RAM, apps: &[Application], state: &mut State) -> Result<()> {
        for app in apps {
            for path in &app.paths {
                Handler::_restore(path, ram, state)?
            }
        }
        Ok(())
    }

    pub fn restore<P: AsRef<Path>>(&mut self, t_path: P) -> Result<()> {
        Handler::_restore(t_path, &self.ram, &mut self.state)?;
        Ok(())
    }

    fn _restore<P: AsRef<Path>>(t_path: P, ram: &RAM, state: &mut State) -> anyhow::Result<()> {
        let t_path: &Path = t_path.as_ref();
        let t_dir = t_path.parent().with_context(|| "There isn't parent path")?;

        let s_path = AbsPath::try_from(&ram.mount_path)?
            .join(&ram.name)?
            .join(&t_path)?;

        std::fs::remove_file(t_path).with_context(|| "Cannot Delete file")?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(&s_path, t_dir, &option)?;

        state.remove_and_save(&t_path)?;
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        let sp = env::get_state_path();
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
        std::env::set_var(crate::env::KEY_STATE_PATH, path);

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
        let mut state = State::load();

        // Backup
        check!(Handler::mount(&ram));
        check!(Handler::_backup(target_str, &ram, &mut state));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), true);
        assert_eq!(m.file_type().is_dir(), false);

        // Is Correct SymLink
        //        let sym_file_path = mount_path
        //            .join(&ram.name)
        //            .join(target_path.strip_prefix("/").unwrap());
        //        assert_eq!(sym_file_path, check!(fs::read_link(target_str)));

        // Restore
        check!(Handler::_restore(target_str, &ram, &mut state));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), false);
        assert_eq!(m.file_type().is_dir(), true);

        check!(Handler::unmount(&ram));
    }
}

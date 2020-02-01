use crate::application::Application;
use crate::config::{Config, RAM};
use crate::error::Result;
use crate::maccmd::{DiskUtil, HdiUtil};
use crate::path::AbsPath;
use crate::state::State;
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use shellexpand;
use std::convert::TryFrom;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Ramup {
    ram: RAM,
    applications: Vec<Application>,
    state: State,
}

impl Ramup {
    #[allow(dead_code)]
    pub fn from_file() -> Result<Ramup> {
        let config: Config = Config::new();
        let state: State = State::new_from_file()?;
        Ok(Ramup {
            ram: config.ram,
            applications: config.applications,
            state,
        })
    }

    #[allow(dead_code)]
    pub fn from_str(contents: &str) -> Result<Ramup> {
        let config: Config = toml::from_str(contents).unwrap();
        let state: State = State::new();
        Ok(Ramup {
            ram: config.ram,
            applications: config.applications,
            state,
        })
    }

    #[allow(dead_code)]
    pub fn backup_all(&self) -> Result<()> {
        for app in &self.applications {
            for path in &app.paths {
                self.backup(path)?
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn backup<P: AsRef<Path>>(&self, s_path: P) -> Result<()> {
        self.mount()?;

        let t_path = AbsPath::try_from(&self.ram.mount_path)?
            .join(&self.ram.name)?
            .join(&s_path)?;
        let t_dir = t_path.parent()?;

        std::fs::create_dir_all(&t_dir)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(&s_path, &t_dir, &option)?;
        std::os::unix::fs::symlink(t_path, s_path)?;

        //        self.state.add_and_save(path);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn restore_all(&self) -> Result<()> {
        for app in &self.applications {
            for path in &app.paths {
                self.restore(path)?
            }
        }
        self.unmount()
    }

    #[allow(dead_code)]
    pub fn restore<P: AsRef<Path>>(&self, t_path: P) -> Result<()> {
        let t_path: &Path = t_path.as_ref();
        let t_dir = t_path.parent().with_context(|| "There isn't parent path")?;

        let s_path = AbsPath::try_from(&self.ram.mount_path)?
            .join(&self.ram.name)?
            .join(&t_path)?;

        std::fs::remove_file(t_path)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(s_path, t_dir, &option)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn clean(&self) -> Result<()> {
        self.unmount()
    }

    fn mount(&self) -> Result<()> {
        if HdiUtil::exist_volume(&self.ram.name)? {
            return Ok(());
        }
        let mountpoint = HdiUtil::attach(self.ram.size)?;
        DiskUtil::erasevolume(&self.ram.name, &mountpoint)
    }

    fn unmount(&self) -> Result<()> {
        if !HdiUtil::exist_volume(&self.ram.name)? {
            return Ok(());
        }
        HdiUtil::detach_volume(&self.ram.name)
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
    fn from_str() {
        let t = r#"
        [ram]
        name = "RAMDisk"
        size = 8388607

        [[applications]]
        name = "example"
        restart = false
        "#;
        let ramup = check!(Ramup::from_str(t));
        assert_eq!(ramup.ram.name, "RAMDisk".to_string())
    }

    #[test]
    #[serial]
    #[cfg(target_os = "macos")]
    fn backup_and_restore() {
        let mount_tmp_dir = check!(TempDir::new("ramup-volume-ram"));
        let mount_path = mount_tmp_dir.path();
        let mount_str = mount_path.to_str().unwrap();

        let target_tmp_dir = check!(TempDir::new("ramup-target"));
        let target_path = target_tmp_dir.path();
        let target_str = target_path.to_str().unwrap();

        let dir = TempDir::new("ramup-for-test").unwrap();
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
        let ramup = check!(Ramup::from_str(&toml));

        // Backup
        check!(ramup.backup(target_str));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), true);
        assert_eq!(m.file_type().is_dir(), false);

        // Is Correct SymLink
        let sym_file_path = mount_path
            .join(&ramup.ram.name)
            .join(target_path.strip_prefix("/").unwrap());
        assert_eq!(sym_file_path, check!(fs::read_link(target_str)));

        // Restore
        check!(ramup.restore(target_str));
        let m = check!(fs::symlink_metadata(target_str));
        assert_eq!(m.file_type().is_symlink(), false);
        assert_eq!(m.file_type().is_dir(), true);

        ramup.clean().unwrap();
    }
}

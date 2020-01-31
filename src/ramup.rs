use crate::application::Application;
use crate::config::{Config, RAM};
use crate::error::Result;
use crate::maccmd::{DiskUtil, HdiUtil};
use crate::state::State;
use fs_extra::dir::CopyOptions;
use shellexpand;
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
        let state: State = State::new();
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
        self.mount()?;
        for app in &self.applications {
            for path in &app.paths {
                self.backup(path.as_str())?
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn backup(&self, path: &str) -> Result<()> {
        self.mount()?;
        let path = shellexpand::tilde(path).to_string();
        let path = Path::new(&path);
        let ram_path = Path::new(&self.ram.mount_path)
            .join(&self.ram.name)
            .join(path.strip_prefix("/")?);
        let ram_parent_path = ram_path.parent().unwrap();

        std::fs::create_dir_all(ram_parent_path)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(path, ram_parent_path, &option)?;
        std::os::unix::fs::symlink(ram_path, path)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn restore_all(&self) -> Result<()> {
        for app in &self.applications {
            for path in &app.paths {
                self.restore(path.as_str())?
            }
        }
        self.unmount()
    }

    #[allow(dead_code)]
    pub fn restore(&self, path: &str) -> Result<()> {
        let path = Path::new(path);
        let parent_path = path.parent().unwrap();
        let ram_path = Path::new(&self.ram.mount_path)
            .join(&self.ram.name)
            .join(path.strip_prefix("/")?);

        std::fs::remove_file(path)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(ram_path, parent_path, &option)?;
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

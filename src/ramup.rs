use crate::application::Application;
use crate::config::{Config, RAM};
use crate::error::{AppError, Result};
use crate::utils;
use fs_extra::dir::CopyOptions;
use shellexpand::full;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default)]
pub struct Ramup {
    ram: RAM,
    applications: Vec<Application>,
}

impl Ramup {
    pub fn from_file(path: &str) -> Result<Ramup> {
        let path = shellexpand::tilde("~/.config/ramup/config.toml").to_string();
        let mut contents = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents).unwrap();
        Ok(Ramup {
            ram: config.ram,
            applications: config.applications,
        })
    }

    pub fn from_str(contents: &str) -> Result<Ramup> {
        let config: Config = toml::from_str(contents).unwrap();
        Ok(Ramup {
            ram: config.ram,
            applications: config.applications,
        })
    }

    pub fn backup(&self, path: &str) -> Result<()> {
        let path = Path::new(path);
        let ram_path = Path::new(&self.ram.mount_path).join(path.strip_prefix("/")?);
        let ram_parent_path = ram_path.parent().unwrap();

        std::fs::create_dir_all(ram_parent_path)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(path, ram_parent_path, &option);
        std::os::unix::fs::symlink(ram_path, path)?;
        Ok(())
    }

    pub fn restore(&self, path: &str) -> Result<()> {
        let path = Path::new(path);
        let parent_path = path.parent().unwrap();
        let ram_path = Path::new(&self.ram.mount_path).join(path.strip_prefix("/")?);

        std::fs::remove_file(path)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(ram_path, parent_path, &option)?;
        Ok(())
    }

    //    pub fn new_old(config: Config) -> Ramup {
    //        Ramup {
    //            config,
    //            mount_point: "".to_string(),
    //        }
    //    }
    //
    //    pub fn create_old(&mut self) -> clap::Result<()> {
    //        self.mount_point = utils::mount(&self.config.ram.size, &self.config.ram.name)?;
    //        Ok(())
    //    }
    //
    //    pub fn backup_old(&mut self) {
    //        println!("backup start");
    //        for app in &mut self.config.applications {
    //            app.backup(&self.config.ram.name);
    //        }
    //        println!("backup finished");
    //    }
    //
    //    pub fn restore_old(&self) {
    //        println!("restore start");
    //        for app in &self.config.applications {
    //            app.restore(&self.config.ram.name);
    //        }
    //        Command::new("hdiutil")
    //            .args(&["detach", &self.mount_point])
    //            .output()
    //            .expect("detach is failed");
    //        println!("restore finished");
    //    }
    //
    //    pub fn rsync_old(&self) {
    //        println!("rsync start");
    //        for app in &self.config.applications {
    //            app.rsync(&self.config.ram.name);
    //        }
    //        println!("rsync end");
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn from_str() {
        let t = r#"
        [ram]
        devname = "RAMDisk"
        size = 8388607

        [[applications]]
        name = "example"
        restart = false
        "#;
        let ramup = Ramup::from_str(t).unwrap();
        assert_eq!(ramup.ram.devname, "RAMDisk".to_string())
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn backup() {
        let t = r#"
        [ram]
        devname = "RAMDisk"
        size = 8388607
        mount_path = "/tmp"
        "#;
        let ramup = Ramup::from_str(t).unwrap();
        ramup.backup("/tmp/hoge/fuga");
        ramup.restore("/tmp/hoge/fuga").expect("restore failed");
    }
}

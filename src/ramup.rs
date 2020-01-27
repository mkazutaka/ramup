use crate::config::Config;
use crate::utils;
use clap::Result;
use fs_extra::dir::{move_dir, CopyOptions};
use shellexpand;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Ramup {
    config: Config,
    mount_point: String,
}

impl Ramup {
    pub fn new(config: Config) -> Ramup {
        Ramup {
            config,
            mount_point: "".to_string(),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        self.mount_point = utils::mount(&self.config.ram.size, &self.config.ram.name)?;
        Ok(())
    }

    pub fn backup(&mut self) {
        for app in &mut self.config.applications {
            for path in &app.paths {
                println!("backup: {}", path.as_str());
                let path = shellexpand::tilde(&path).to_string();
                let path = Path::new(path.as_str());

                let dir_path = path.parent().unwrap();
                let dir_path = dir_path.to_str().unwrap();
                let app_path = path.to_str().unwrap();
                let ram_dir_path = format!("/Volumes/{}{}", self.config.ram.name, dir_path);
                let ram_app_path = format!("/Volumes/{}{}", self.config.ram.name, app_path);

                let mut option = CopyOptions::new();
                option.copy_inside = true;

                fs::create_dir_all(&ram_dir_path).unwrap();
                move_dir(&app_path, &ram_dir_path, &option).unwrap();
                std::os::unix::fs::symlink(&ram_app_path, &app_path).unwrap();
            }
        }
    }

    pub fn restore(&self) {
        for app in &self.config.applications {
            for path in &app.paths {
                println!("restore: {}", path.as_str());
                let path = shellexpand::tilde(&path).to_string();
                let path = Path::new(path.as_str());

                let dir_path = path.parent().unwrap();
                let dir_path = dir_path.to_str().unwrap();
                let app_path = path.to_str().unwrap();
                let _ram_dir_path = format!("/Volumes/{}{}", self.config.ram.name, dir_path);
                let ram_app_path = format!("/Volumes/{}{}", self.config.ram.name, app_path);

                fs::remove_file(&app_path).unwrap();
                let mut option = CopyOptions::new();
                option.copy_inside = true;
                move_dir(&ram_app_path, &dir_path, &option).unwrap();

                Command::new("hdiutil")
                    .args(&["detach", &self.mount_point])
                    .output()
                    .expect("detach is failed");
            }
        }
    }
}

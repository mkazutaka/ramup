use crate::config::Config;
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
    pub fn new(user_config: Config) -> Ramup {
        Ramup {
            config: user_config,
            mount_point: "".to_string(),
        }
    }

    fn set_mount_point(&mut self, mount_point: impl Into<String>) {
        self.mount_point = mount_point.into();
    }

    /// diskutil erasevolume HFS+ RAMUp `hdiutil attach -nomount ram://4194304`
    pub fn create(&mut self) -> Result<()> {
        let image = format!("ram://{}", self.config.ram.size);
        let mount_point = Command::new("hdiutil")
            .args(&["attach", "-nomount", &image])
            .output()?;

        let mount_point = String::from_utf8(mount_point.stdout).unwrap();
        let mount_point = mount_point.trim();

        let volume = self.config.ram.name.as_str();
        Command::new("diskutil")
            .args(&["erasevolume", "HFS+", volume, mount_point])
            .output()?;

        self.set_mount_point(mount_point);
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

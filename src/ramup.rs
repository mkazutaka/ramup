use crate::config::Config;
use fs_extra::dir::{move_dir, CopyOptions};
use shellexpand;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Ramup {
    mount_point: String,
    user_config: Config,
}

impl Ramup {
    pub fn new(user_config: Config) -> Ramup {
        Ramup {
            mount_point: "".to_string(),
            user_config,
        }
    }

    fn set_mount_point(&mut self, mount_point: impl Into<String>) {
        self.mount_point = mount_point.into();
    }

    /// diskutil erasevolume HFS+ RAMUp `hdiutil attach -nomount ram://4194304`
    pub fn create(&mut self) {
        let mount_point = Command::new("hdiutil")
            .args(&["attach", "-nomount", "ram://8388608"])
            .output()
            .expect("attach is failed");

        let mount_point = String::from_utf8(mount_point.stdout).unwrap();
        let mount_point = mount_point.trim();

        Command::new("diskutil")
            .args(&["erasevolume", "HFS+", "RAMUpDisk", mount_point])
            .output()
            .expect("diskutil is failed");

        self.set_mount_point(mount_point);
    }

    pub fn backup(&mut self) {
        for user_config in &mut self.user_config.applications {
            match &user_config.paths {
                Some(paths) => {
                    for path in paths {
                        println!("backup: {}", path.as_str());
                        let path = shellexpand::tilde(path).to_string();
                        let path = Path::new(path.as_str());

                        let dir_path = path.parent().unwrap();
                        let dir_path = dir_path.to_str().unwrap();
                        let app_path = path.to_str().unwrap();
                        let ram_dir_path = format!("/Volumes/RAMUpDisk{}", dir_path);
                        let ram_app_path = format!("/Volumes/RAMUpDisk{}", app_path);

                        let mut option = CopyOptions::new();
                        option.copy_inside = true;

                        fs::create_dir_all(&ram_dir_path).unwrap();
                        move_dir(&app_path, &ram_dir_path, &option).unwrap();
                        std::os::unix::fs::symlink(&ram_app_path, &app_path).unwrap();
                    }
                }
                None => {}
            };
        }
    }

    pub fn restore(&self) {
        for user_config in &self.user_config.applications {
            match &user_config.paths {
                Some(paths) => {
                    for path in paths {
                        println!("restore: {}", path.as_str());
                        let path = shellexpand::tilde(path).to_string();
                        let path = Path::new(path.as_str());

                        let dir_path = path.parent().unwrap();
                        let dir_path = dir_path.to_str().unwrap();
                        let app_path = path.to_str().unwrap();
                        let _ram_dir_path = format!("/Volumes/RAMUpDisk{}", dir_path);
                        let ram_app_path = format!("/Volumes/RAMUpDisk{}", app_path);

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
                None => {}
            };
        }
    }
}

use crate::application::DefaultApplicationConfig;
use crate::config::UserConfig;
use shellexpand;
use std::path::Path;
use std::process::Command;

pub struct Ramup {
    mount_point: String,
    user_config: UserConfig,
}

impl Ramup {
    pub fn new(user_config: UserConfig) -> Ramup {
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
            .args(&["attach", "-nomount", "ram://4194304"])
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
            match &user_config.files {
                Some(files) => {
                    for path in files {
                        let path = shellexpand::tilde(path).to_string();
                        let path = Path::new(path.as_str());

                        let dir_path = path.parent().unwrap();
                        let dir_path = dir_path.to_str().unwrap();
                        let app_path = path.to_str().unwrap();
                        let ram_dir_path = format!("/Volumes/RAMUpDisk{}", dir_path);
                        let ram_app_path = format!("/Volumes/RAMUpDisk{}", app_path);

                        Command::new("mkdir")
                            .args(&["-p", &ram_dir_path])
                            .output()
                            .expect("mkdir is failed");
                        Command::new("mv")
                            .args(&[app_path, &ram_app_path])
                            .output()
                            .expect("mv is failed");
                        Command::new("ln")
                            .args(&["-s", &ram_app_path, app_path])
                            .output()
                            .expect("ln is failed");
                    }
                }
                None => {}
            };
        }
    }

    pub fn restore(&self) {
        for user_config in &self.user_config.applications {
            match &user_config.files {
                Some(files) => {
                    for path in files {
                        let path = shellexpand::tilde(path).to_string();
                        let path = Path::new(path.as_str());

                        let dir_path = path.parent().unwrap();
                        let dir_path = dir_path.to_str().unwrap();
                        let app_path = path.to_str().unwrap();
                        let ram_dir_path = format!("/Volumes/RAMUpDisk{}", dir_path);
                        let ram_app_path = format!("/Volumes/RAMUpDisk{}", app_path);

                        if path.is_dir() {
                            Command::new("unlink")
                                .args(&[&app_path])
                                .output()
                                .expect("unlink is failed");
                            Command::new("mv")
                                .args(&[&ram_app_path, app_path])
                                .output()
                                .expect("mv is failed");
                            Command::new("hdiutil")
                                .args(&["detach", &self.mount_point])
                                .output()
                                .expect("detach is failed");
                        }
                    }
                }
                None => {}
            };
        }
    }
}

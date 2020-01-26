use serde::private::ser::constrain;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub applications: Vec<UserApplicationConfig>,
}

#[derive(Debug, Deserialize)]
pub struct UserApplicationConfig {
    pub name: String,
    pub restart: Option<bool>,
    pub files: Option<Vec<String>>,
}

impl UserApplicationConfig {
    pub fn set_restart(&mut self, restart: bool) {
        self.restart = Some(restart);
    }

    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = Some(files);
    }
}

impl UserConfig {
    pub fn new() -> UserConfig {
        let path = "/Users/mkazutaka/.config/ramup/config.toml";

        let mut contents = String::new();
        File::open(&path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        toml::from_str(&contents).unwrap()
    }
}

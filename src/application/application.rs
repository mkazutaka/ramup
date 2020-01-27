use crate::application::ApplicationVisitor;
use fs_extra::dir::{move_dir, CopyOptions};
use rust_embed::RustEmbed;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Application {
    pub name: String,
    pub restart: Option<bool>,
    pub paths: Vec<String>,
}

impl Application {
    pub fn backup(&self, ram: &str) {
        for path in &self.paths {
            println!("backup: {}", path.as_str());
            let path = shellexpand::tilde(&path).to_string();
            let path = Path::new(path.as_str());

            let dir_path = path.parent().unwrap();
            let dir_path = dir_path.to_str().unwrap();
            let app_path = path.to_str().unwrap();
            let ram_dir_path = format!("/Volumes/{}{}", ram, dir_path);
            let ram_app_path = format!("/Volumes/{}{}", ram, app_path);

            let mut option = CopyOptions::new();
            option.copy_inside = true;

            fs::create_dir_all(&ram_dir_path).unwrap();
            move_dir(&app_path, &ram_dir_path, &option).unwrap();
            std::os::unix::fs::symlink(&ram_app_path, &app_path).unwrap();
        }
    }

    pub fn restore(&self, ram: &str) {
        for path in &self.paths {
            println!("restore: {}", path.as_str());
            let path = shellexpand::tilde(&path).to_string();
            let path = Path::new(path.as_str());

            let dir_path = path.parent().unwrap();
            let dir_path = dir_path.to_str().unwrap();
            let app_path = path.to_str().unwrap();
            let _ram_dir_path = format!("/Volumes/{}{}", ram, dir_path);
            let ram_app_path = format!("/Volumes/{}{}", ram, app_path);

            fs::remove_file(&app_path).unwrap();
            let mut option = CopyOptions::new();
            option.copy_inside = true;
            move_dir(&ram_app_path, &dir_path, &option).unwrap();
        }
    }
}

impl<'de> Deserialize<'de> for Application {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ApplicationVisitor)
    }
}

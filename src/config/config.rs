use crate::config::{ApplicationConfig, RAMConfig};
use serde::Deserialize;
use shellexpand;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub applications: Vec<ApplicationConfig>,
    pub ram: RAMConfig,
}

impl Config {
    pub fn new() -> Config {
        let path = "~/.config/ramup/config.toml";
        let path = shellexpand::tilde(path).to_string();

        let mut contents = String::new();
        File::open(&path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        toml::from_str(&contents).unwrap()
    }
}

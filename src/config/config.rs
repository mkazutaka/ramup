use crate::application::Application;
use crate::config::RAM;
use serde::Deserialize;
use shellexpand;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub ram: RAM,
    pub applications: Vec<Application>,
}

impl Config {
    pub fn new() -> Config {
        let path = "~/.config/ramup/config.toml";
        let path = shellexpand::tilde(path).to_string();

        let mut contents = String::new();
        File::open(&path)
            .expect("cannot open file")
            .read_to_string(&mut contents)
            .expect("cannot read string from file");

        toml::from_str(&contents).expect("cannot read from toml config")
    }
}

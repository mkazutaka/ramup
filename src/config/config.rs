use crate::application::Application;
use crate::config::RAM;
use crate::env;
use crate::error::Result;
use serde::Deserialize;
//use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub ram: RAM,
    pub applications: Vec<Application>,
}

impl Config {
    pub fn new() -> Self {
        let cp = env::get_config_path();
        let c = fs::read_to_string(&cp).unwrap();
        let config: Config = toml::from_str(&c).unwrap();
        config
    }

    pub fn initialize() -> Result<()> {
        let initial = r#"
[ram]
name = "RAMDiskByRamup"
size = 8388608

# Add your's first application
# [[applications]]
# name = "example"
        "#;

        let cp = env::get_config_path();
        let mut file = File::create(&cp)?;
        file.write_all(initial.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs::File;
    use std::io::prelude::*;
    use tempdir::TempDir;

    #[test]
    #[serial]
    fn create_file() {
        let dir = TempDir::new("ramup_for_test").unwrap();
        let config = dir.path().join("config.toml");
        std::env::set_var(
            env::KEY_CONFIG_PATH,
            dir.path().join("config.toml").to_str().unwrap(),
        );
        Config::initialize().unwrap();

        let mut file = File::open(&config).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        assert_eq!(config.ram.name, "RAMDiskByRamup");
        assert_eq!(config.ram.size, 8388608);

        std::env::remove_var(env::KEY_CONFIG_PATH);
    }
}

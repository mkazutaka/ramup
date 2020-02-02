use crate::application::Application;
use crate::env;
use crate::ram::RAM;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static DEFAULT_CONFIG: &str = r#"
    [ram]
    name = "RAMDiskByRamup"
    size = 8388608

    # Add your's first application
    # [[applications]]
    # name = "example"
"#;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub ram: RAM,
    pub applications: Vec<Application>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config: Config = toml::from_str(DEFAULT_CONFIG)?;
        Ok(config)
    }

    pub fn load() -> Result<Self> {
        let cp = env::get_config_path();
        match Path::new(&cp).exists() {
            true => {
                let c = fs::read_to_string(&cp)?;
                let config: Config = toml::from_str(&c)?;
                Ok(config)
            }
            false => Config::new(),
        }
    }

    pub fn initialize() -> Result<()> {
        let cp = env::get_config_path();
        if Path::new(&cp).exists() {
            return Ok(());
        }
        let cp = env::get_config_path();
        let mut file = File::create(&cp)?;
        file.write_all(DEFAULT_CONFIG.as_bytes())?;
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

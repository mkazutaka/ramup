use crate::application::Application;
use crate::config::RAM;
use crate::error::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub ram: RAM,
    pub applications: Vec<Application>,
}

impl Config {
    pub fn init<P: AsRef<Path>>(path: &P) -> Result<()> {
        let initial = r#"
[ram]
name = "RAMDiskByRamup"
size = 8388608

# Add your's first application
# [[applications]]
# name = "example"
        "#;

        let mut file = File::create(&path)?;
        file.write_all(initial.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn create_file() {
        use std::fs::File;
        use std::io::prelude::*;

        let dir = TempDir::new("ramup_for_test").unwrap();
        let config = dir.path().join("config.toml");
        Config::init(&config).unwrap();

        let mut file = File::open(&config).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();
        assert_eq!(config.ram.name, "RAMDiskByRamup");
        assert_eq!(config.ram.size, 8388608)
    }
}

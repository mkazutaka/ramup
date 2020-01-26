use crate::applications::example::Example;
use crate::applications::DefaultApplicationConfig;
use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::private::ser::constrain;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub applications: Vec<UserApplicationConfig>,
}

#[derive(Debug)]
pub struct UserApplicationConfig {
    pub name: String,
    pub restart: Option<bool>,
    pub files: Option<Vec<String>>,
}

impl UserApplicationConfig {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_restart(&mut self, restart: bool) {
        self.restart = Some(restart);
    }

    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = Some(files);
    }
}

struct UserApplicationConfigVisitor;

impl<'de> Visitor<'de> for UserApplicationConfigVisitor {
    type Value = UserApplicationConfig;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("a very special map")
    }

    fn visit_map<V>(self, mut map: V) -> Result<UserApplicationConfig, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut app_config = UserApplicationConfig {
            name: "".to_string(),
            restart: None,
            files: None,
        };

        while let Some(key) = map.next_key()? {
            match key {
                "name" => app_config.set_name(map.next_value().unwrap()),
                "restart" => app_config.set_restart(map.next_value().unwrap()),
                "files" => app_config.set_files(map.next_value().unwrap()),
                _ => {}
            }
        }

        let default_config = DefaultApplicationConfig::from(&app_config.name);
        if app_config.restart.is_none() {
            app_config.restart = Some(default_config.restart);
        }
        if app_config.files.is_none() {
            app_config.files = Some(default_config.files);
        }

        Ok(app_config)
    }
}

impl<'de> Deserialize<'de> for UserApplicationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(UserApplicationConfigVisitor)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let config = r#"
        name = "example"
        restart = false
        "#;

        let config: UserApplicationConfig = toml::from_str(config).unwrap();
        assert_eq!(config.name, "example");
    }
}

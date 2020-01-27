pub mod example;

use crate::applications::example::EXAMPLE_TOML;
use crate::config::UserApplicationConfig;
use clap::App;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DefaultApplicationConfig {
    pub name: String,
    pub restart: bool,
    pub files: Vec<String>,
}

impl DefaultApplicationConfig {
    pub fn from(name: &String) -> DefaultApplicationConfig {
        let toml = match name.as_str() {
            "example" => EXAMPLE_TOML,
            _ => {
                r#"
name = ""
restart = false
files = []
            "#
            }
        };

        let c: DefaultApplicationConfig = toml::from_str(toml).unwrap();
        c
    }
}

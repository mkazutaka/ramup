pub mod example;

use crate::applications::example::Example;
use crate::config::UserApplicationConfig;
use clap::App;

trait Application {}

pub struct DefaultApplicationConfig {
    pub name: String,
    pub restart: bool,
    pub files: Vec<String>,
}

impl DefaultApplicationConfig {
    pub fn create(name: &String) -> DefaultApplicationConfig {
        match name.as_str() {
            "example" => DefaultApplicationConfig::new_example(),
            _ => DefaultApplicationConfig {
                name: "".to_string(),
                restart: false,
                files: vec![],
            },
        }
    }

    pub fn new_example() -> Self {
        DefaultApplicationConfig {
            name: "example".into(),
            restart: false,
            files: vec!["/Users/mkazutaka/example".into()],
        }
    }
}

use crate::config::UserApplicationConfig;
use clap::App;
use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DefaultApplicationConfig {
    pub name: String,
    pub restart: bool,
    pub files: Vec<String>,
}

#[derive(RustEmbed)]
#[folder = "applications/"]
struct Assets;

impl DefaultApplicationConfig {
    pub fn from(name: &String) -> DefaultApplicationConfig {
        for file in Assets::iter() {
            if format!("{}.toml", name) == file.as_ref() {
                let file = file.as_ref();
                let file = Assets::get(file).unwrap();
                let file = file.as_ref();

                let toml_content = std::str::from_utf8(file).unwrap();
                let c: DefaultApplicationConfig = toml::from_str(toml_content).unwrap();
                return c;
            }
        }

        let mut toml_content = r#"
            name = ""
            restart = false
            files = []
        "#;
        let c: DefaultApplicationConfig = toml::from_str(toml_content).unwrap();
        c
    }
}

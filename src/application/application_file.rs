use rust_embed::RustEmbed;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "applications/"]
struct ApplicationFiles;

#[derive(Deserialize)]
pub struct ApplicationFile {
    pub name: String,
    pub restart: bool,
    pub paths: Vec<String>,
}

impl ApplicationFile {
    pub fn from(name: &String) -> ApplicationFile {
        for file in ApplicationFiles::iter() {
            if format!("{}.toml", name) == file.as_ref() {
                let file = file.as_ref();
                let file = ApplicationFiles::get(file).unwrap();
                let file = file.as_ref();

                let toml_content = std::str::from_utf8(file).unwrap();
                let c: ApplicationFile = toml::from_str(toml_content).unwrap();
                return c;
            }
        }

        let toml_content = r#"
            name = ""
            restart = false
            paths = []
        "#;
        let c: ApplicationFile = toml::from_str(toml_content).unwrap();
        c
    }
}

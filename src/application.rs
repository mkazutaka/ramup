use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Application {
    pub name: String,
    pub restart: bool,
    pub paths: Vec<String>,
}

#[derive(RustEmbed)]
#[folder = "applications/"]
struct Applications;

impl Application {
    pub fn from(name: &String) -> Application {
        for file in Applications::iter() {
            if format!("{}.toml", name) == file.as_ref() {
                let file = file.as_ref();
                let file = Applications::get(file).unwrap();
                let file = file.as_ref();

                let toml_content = std::str::from_utf8(file).unwrap();
                let c: Application = toml::from_str(toml_content).unwrap();
                return c;
            }
        }

        let toml_content = r#"
            name = ""
            restart = false
            paths = []
        "#;
        let c: Application = toml::from_str(toml_content).unwrap();
        c
    }
}

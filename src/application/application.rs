use crate::application::ApplicationVisitor;
use rust_embed::RustEmbed;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Application {
    pub name: String,
    pub restart: Option<bool>,
    pub paths: Vec<String>,
}

impl Application {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_restart(&mut self, restart: bool) {
        self.restart = Some(restart);
    }

    pub fn set_paths(&mut self, paths: Vec<String>) {
        self.paths = paths;
    }
}

impl<'de> Deserialize<'de> for Application {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ApplicationVisitor)
    }
}

use crate::application::ApplicationVisitor;
use serde::de::Deserializer;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Application {
    pub name: String,
    pub restart: Option<bool>,
    pub paths: Vec<String>,
}

impl Application {
    #[allow(dead_code)]
    pub fn rsync(&self, ram: &str) {
        for path in &self.paths {
            println!("restore: {}", path.as_str());
            let path = shellexpand::tilde(&path).to_string();

            let source = format!("/Volumes/{}{}", ram, path);
            let destination = format!("/Users/mkazutaka/.config/ramup/backup{}", path);
            let console_info = rusync::ConsoleProgressInfo::new();
            let options = rusync::SyncOptions::new();
            let source = Path::new(source.as_str());
            let destination = Path::new(destination.as_str());

            let syncer =
                rusync::Syncer::new(&source, &destination, options, Box::new(console_info));
            let _stats = syncer.sync();
        }
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

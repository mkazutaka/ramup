use crate::application::{Application, ApplicationFile};
use rust_embed::RustEmbed;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::Deserialize;

pub struct ApplicationVisitor;

impl<'de> Visitor<'de> for ApplicationVisitor {
    type Value = Application;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("a very special map")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Application, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut app_config = Application {
            name: "".to_string(),
            restart: None,
            paths: vec![],
        };

        while let Some(key) = map.next_key()? {
            match key {
                "name" => app_config.set_name(map.next_value().unwrap()),
                "restart" => app_config.set_restart(map.next_value().unwrap()),
                "paths" => app_config.set_paths(map.next_value().unwrap()),
                _ => {}
            }
        }

        let default_config = ApplicationFile::from(&app_config.name);
        if app_config.restart.is_none() {
            app_config.restart = Some(default_config.restart);
        }
        if app_config.paths.len() == 0 {
            app_config.paths = default_config.paths;
        }

        Ok(app_config)
    }
}

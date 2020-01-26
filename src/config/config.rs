use crate::application::Application;
use crate::config::RAM;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub ram: RAM,
    pub applications: Vec<Application>,
}

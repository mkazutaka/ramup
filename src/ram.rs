use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct RAM {
    pub size: isize,
    pub name: String,
    pub mount_path: String,
}

impl Default for RAM {
    fn default() -> Self {
        RAM {
            size: 8_388_608,
            name: "RAMDiskbyRamup".into(),
            mount_path: "/Volumes".into(),
        }
    }
}

impl RAM {
    #[allow(dead_code)]
    pub fn new_from_str(c: &str) -> Result<Self> {
        let ram: RAM = toml::from_str(c)?;
        Ok(ram)
    }
}

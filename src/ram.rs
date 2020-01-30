use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct RAMConfig {
    pub size: isize,
    pub name: String,
}

impl Default for RAMConfig {
    fn default() -> Self {
        RAMConfig {
            size: 8388608,
            name: "RAMDisk by ramup".into(),
        }
    }
}

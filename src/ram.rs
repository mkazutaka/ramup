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
            name: "RAMDisk by ramup".into(),
            mount_path: "/Volumes".into(),
        }
    }
}

use crate::applications::DefaultApplicationConfig;

pub struct Example {}

impl Example {
    pub fn create() -> DefaultApplicationConfig {
        DefaultApplicationConfig {
            name: "example".into(),
            restart: false,
            files: vec!["~/example".into()],
        }
    }
}

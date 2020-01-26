use crate::applications::Application;
use crate::config::UserApplicationConfig;

pub struct Example {
    name: String,
    restart: bool,
    files: Vec<String>,
}

impl Example {
    pub fn new() -> Self {
        Example {
            name: "example".to_string(),
            restart: false,
            files: vec!["~/example".into()],
        }
    }

    //    pub fn create_user_application_config(
    //        setting: &UserApplicationConfig,
    //    ) -> UserApplicationConfig {
    //        let default = Example::new();
    //
    //        let restart = setting.restart.unwrap_or(default.restart).clone();
    //        let files = setting.files.unwrap_or(default.files).clone();
    //
    //        UserApplicationConfig {
    //            name: format!("{}", setting.name),
    //            restart: Some(restart),
    //            files: Some(files),
    //        }
    //    }
}

impl Application for Example {}

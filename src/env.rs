use std::env;

pub static KEY_CONFIG_PATH: &str = "RAMUP_CONFIG_PATH";
pub static KEY_STATE_PATH: &str = "RAMUP_CONFIG_PATH";

#[allow(dead_code)]
pub fn get_config_path() -> String {
    let default = "~/.config/ramup/config.toml";
    env::var(KEY_CONFIG_PATH).unwrap_or_else(|_| String::from(default))
}

#[allow(dead_code)]
pub fn get_state_path() -> String {
    let default = "~/.config/ramup/state.toml";
    env::var(KEY_STATE_PATH).unwrap_or_else(|_| String::from(default))
}

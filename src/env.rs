use shellexpand;
use std::env;

pub static KEY_CONFIG_PATH: &str = "RAMUP_CONFIG_PATH";
pub static KEY_STATE_PATH: &str = "RAMUP_CONFIG_PATH";

#[allow(dead_code)]
pub fn get_config_path() -> String {
    let default = shellexpand::tilde("~/.config/ramup/config.toml");
    env::var(KEY_CONFIG_PATH).unwrap_or_else(|_| String::from(default))
}

#[allow(dead_code)]
pub fn get_state_path() -> String {
    let default = shellexpand::tilde("~/.config/ramup/state.toml");
    env::var(KEY_STATE_PATH).unwrap_or_else(|_| String::from(default))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn config_path() {
        let t_home = env::var("HOME").unwrap();
        env::remove_var(KEY_CONFIG_PATH);
        env::remove_var(KEY_STATE_PATH);
        env::set_var("HOME", "/home");

        assert_eq!("/home/.config/ramup/config.toml", get_config_path());
        assert_eq!("/home/.config/ramup/state.toml", get_state_path());

        env::set_var("HOME", t_home);
    }
}

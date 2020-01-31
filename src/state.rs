use crate::env;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    backup_paths: Vec<String>,
}

impl State {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let sp = env::get_state_path();
        if !Path::new(&sp).exists() {
            return State::default();
        }
        let c = fs::read_to_string(&sp).unwrap();
        let state: State = toml::from_str(&c).unwrap();
        state
    }

    #[allow(dead_code)]
    pub fn add<P: AsRef<Path>>(&mut self, added: P) {
        self.backup_paths
            .push(added.as_ref().to_string_lossy().into());
        self.save();
    }

    #[allow(dead_code)]
    pub fn remove<P: AsRef<Path>>(&mut self, removed: P) {
        let path = removed.as_ref().to_str().unwrap();
        let index = self.backup_paths.iter().position(|x| *x == path).unwrap();
        self.backup_paths.remove(index);
        self.save();
    }

    #[allow(dead_code)]
    fn save(&self) {
        let sp = env::get_state_path();
        if !Path::new(&sp).exists() {
            fs::File::create(&sp).unwrap();
        }
        let out = toml::to_string(&self).unwrap();
        fs::write(&sp, out).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    const TOML: &str = r#"
backup_paths = [
    "/this/is/path/1",
    "/this/is/path/2",
    "/this/is/path/3"
]
"#;

    #[test]
    fn add() {
        let dir = TempDir::new("ramup-for-test").unwrap();
        let path = dir.path().join("state.toml").to_string_lossy().to_string();
        std::env::set_var(env::KEY_STATE_PATH, path);

        let mut state: State = toml::from_str(&TOML).unwrap();
        state.add("/this/is/new/path");
        assert_eq!("/this/is/new/path", state.backup_paths.last().unwrap());
    }

    #[test]
    fn remove() {
        let dir = TempDir::new("ramup-for-test").unwrap();
        let path = dir.path().join("state.toml").to_string_lossy().to_string();
        std::env::set_var(env::KEY_STATE_PATH, path);

        let mut state: State = toml::from_str(&TOML).unwrap();
        let new_path = Path::new("/this/is/path/3");
        assert_eq!(3, state.backup_paths.len());
        assert_eq!("/this/is/path/3", state.backup_paths.last().unwrap());
        state.remove(new_path);
        assert_eq!(2, state.backup_paths.len());
        assert_eq!("/this/is/path/2", state.backup_paths.last().unwrap());
    }
}

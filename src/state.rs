use crate::env;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    backup_paths: Vec<String>,
}

impl State {
    #[allow(dead_code)]
    pub fn load() -> Self {
        let sp = env::get_state_path();
        if !Path::new(&sp).exists() {
            return State::default();
        }
        let c = fs::read_to_string(&sp).unwrap();
        let state: State = toml::from_str(&c).unwrap();
        state
    }

    #[allow(dead_code)]
    pub fn new_from_file() -> Result<Self> {
        let sp = env::get_state_path();
        if !Path::new(&sp).exists() {
            return Ok(State::default());
        }
        let c = fs::read_to_string(&sp)?;
        let state: State = toml::from_str(&c)?;
        Ok(state)
    }

    #[allow(dead_code)]
    pub fn new_from_str(toml: &str) -> Result<Self> {
        let state: State = toml::from_str(&toml)?;
        Ok(state)
    }

    #[allow(dead_code)]
    pub fn add_and_save<P: AsRef<Path>>(&mut self, added: P) -> Result<()> {
        self.add(added);
        self.save()
    }

    fn add<P: AsRef<Path>>(&mut self, added: P) {
        self.backup_paths
            .push(added.as_ref().to_string_lossy().into());
    }

    #[allow(dead_code)]
    pub fn remove_and_save<P: AsRef<Path>>(&mut self, removed: P) -> Result<()> {
        self.remove(removed)?;
        self.save()
    }

    fn remove<P: AsRef<Path>>(&mut self, removed: P) -> Result<()> {
        let path = removed
            .as_ref()
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("NoneError"))?;
        let index = self
            .backup_paths
            .iter()
            .position(|x| *x == path)
            .ok_or_else(|| anyhow::anyhow!("NoneError"))?;
        self.backup_paths.remove(index);
        Ok(())
    }

    #[allow(dead_code)]
    fn save(&self) -> Result<()> {
        let sp = env::get_state_path();
        if !Path::new(&sp).exists() {
            fs::File::create(&sp)?;
        }
        let out = toml::to_string(&self)?;
        fs::write(&sp, out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempdir::TempDir;

    const TOML: &str = r#"
backup_paths = [
    "/this/is/path/1",
    "/this/is/path/2",
    "/this/is/path/3"
]
"#;

    #[test]
    fn new_from_str() {
        let state = State::new_from_str(TOML).unwrap();
        assert_eq!("/this/is/path/1", state.backup_paths[0])
    }

    #[test]
    fn add() {
        let mut state: State = toml::from_str(&TOML).unwrap();
        state.add("/this/is/new/path");
        assert_eq!("/this/is/new/path", state.backup_paths.last().unwrap());
    }

    #[test]
    fn remove() {
        let mut state: State = toml::from_str(&TOML).unwrap();
        assert_eq!(3, state.backup_paths.len());
        assert_eq!("/this/is/path/3", state.backup_paths.last().unwrap());

        let removed_path = Path::new("/this/is/path/3");
        state.remove(removed_path).unwrap();
        assert_eq!(2, state.backup_paths.len());
        assert_eq!("/this/is/path/2", state.backup_paths.last().unwrap());
    }

    #[test]
    #[serial]
    fn save() {
        let tmp_dir = TempDir::new("ramup").unwrap();
        let tmp_path = tmp_dir
            .path()
            .join("state.toml")
            .to_string_lossy()
            .to_string();
        std::env::set_var(env::KEY_STATE_PATH, tmp_path);

        let state: State = toml::from_str(&TOML).unwrap();
        state.save().unwrap();
        let state = State::load();
        assert_eq!("/this/is/path/3", state.backup_paths.last().unwrap());

        std::env::remove_var(env::KEY_STATE_PATH);
    }
}

use crate::appenv;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    pub backup_paths: Vec<String>,
}

impl State {
    #[allow(dead_code)]
    pub fn load() -> Self {
        let sp = appenv::state();
        if !Path::new(&sp).exists() {
            return State::default();
        }
        let c = fs::read_to_string(&sp).unwrap();
        let state: State = toml::from_str(&c).unwrap();
        state
    }

    #[allow(dead_code)]
    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = String::from(path.as_ref().to_string_lossy());

        if self.backup_paths.iter().any(|s| s == &path) {
            return Ok(());
        };

        self.backup_paths.push(path);
        self.save()
    }

    #[allow(dead_code)]
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = String::from(path.as_ref().to_string_lossy());

        if let Some(index) = self.backup_paths.iter().position(|s| s == &path) {
            self.backup_paths.remove(index);
            self.save()?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn save(&self) -> Result<()> {
        let sp = appenv::state();
        if !Path::new(&sp).exists() {
            let parent = Path::new(&sp).parent().with_context(|| "No Parent")?;
            fs::create_dir_all(parent)?;
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
]
"#;

    fn set_up() {
        let state_path = TempDir::new("ramup").unwrap();
        let state_path = state_path.path().join("state.config");
        std::env::set_var(appenv::KEY_STATE_PATH, state_path);
    }

    #[test]
    #[serial]
    fn add() {
        set_up();

        let mut state: State = State::load();
        assert_eq!(0, state.backup_paths.len());
        state.add("/this/is/new/path").unwrap();

        let state: State = State::load();
        assert_eq!("/this/is/new/path", state.backup_paths.last().unwrap());
    }

    #[test]
    #[serial]
    fn remove() {
        set_up();

        let mut state: State = toml::from_str(&TOML).unwrap();
        assert_eq!("/this/is/path/2", state.backup_paths.last().unwrap());
        state.remove("/this/is/path/2").unwrap();

        let state: State = State::load();
        assert_eq!(1, state.backup_paths.len());
        assert_eq!("/this/is/path/1", state.backup_paths.last().unwrap());
    }
}

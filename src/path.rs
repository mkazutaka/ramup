use anyhow::{Context, Result};
use serde::export::TryFrom;
use shellexpand::tilde;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct AbsPath {
    path: String,
}

impl AbsPath {
    #[allow(dead_code)]
    pub fn new<P: AsRef<Path>>(p: P) -> Result<Self> {
        let p = p.as_ref().to_string_lossy();
        let p = tilde(&p);

        if p.find("~").is_some() {
            return Err(anyhow::anyhow!("Invalid Path"));
        }

        let p = String::from(p);
        let p = Path::new(&p);
        let p = match &p.has_root() {
            true => p.to_path_buf(),
            false => env::current_dir()?.join(p),
        };

        Ok(AbsPath {
            path: String::from(p.to_string_lossy()),
        })
    }

    #[allow(dead_code)]
    pub fn parent(&self) -> Result<Self> {
        let path = Path::new(&self.path)
            .parent()
            .with_context(|| "There isn't parent path")?;

        Ok(AbsPath {
            path: String::from(path.to_string_lossy()),
        })
    }

    #[allow(dead_code)]
    pub fn join<P: AsRef<Path>>(&self, path: P) -> Result<Self> {
        let path = match path.as_ref().has_root() {
            true => {
                let path = path.as_ref().strip_prefix("/")?;
                Path::new(&self.path).join(path)
            }
            false => Path::new(&self.path).join(path),
        };

        Ok(AbsPath {
            path: String::from(path.to_string_lossy()),
        })
    }
}

impl TryFrom<&str> for AbsPath {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        AbsPath::new(value)
    }
}

impl TryFrom<String> for AbsPath {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        AbsPath::new(value)
    }
}

impl TryFrom<&String> for AbsPath {
    type Error = anyhow::Error;

    fn try_from(string: &String) -> Result<Self> {
        AbsPath::new(string)
    }
}

impl TryFrom<&Path> for AbsPath {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self> {
        AbsPath::new(value)
    }
}

impl Into<String> for AbsPath {
    fn into(self) -> String {
        self.path
    }
}

impl ToString for AbsPath {
    fn to_string(&self) -> String {
        String::from(&self.path)
    }
}

impl AsRef<Path> for AbsPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn new() {
        let home = env::var("HOME").unwrap();

        let abs = AbsPath::new("~/./hoge").unwrap();
        assert_eq!(format!("{}/./hoge", home), abs.to_string());

        let current = env::current_dir().unwrap();
        let abs = AbsPath::new("hoge").unwrap();
        assert_eq!(
            format!("{}/hoge", current.to_str().unwrap()),
            abs.to_string()
        );
    }

    #[test]
    #[serial]
    fn from() {
        let home = env::var("HOME").unwrap();

        let abs = AbsPath::try_from("~/./hoge").unwrap();
        assert_eq!(format!("{}/./hoge", home), abs.to_string());
        let abs = AbsPath::try_from(Path::new("~/./hoge")).unwrap();
        assert_eq!(format!("{}/./hoge", home), abs.to_string());
        let abs = AbsPath::try_from(&String::from("~/./hoge")).unwrap();
        assert_eq!(format!("{}/./hoge", home), abs.to_string());
    }

    #[test]
    #[serial]
    fn parent() {
        let mut abs = AbsPath::try_from("~/./hoge").unwrap();
        let abs = abs.parent().unwrap();
        let home = env::var("HOME").unwrap();
        assert_eq!(format!("{}", home), abs.to_string());
    }

    #[test]
    #[serial]
    fn join() {
        let home = env::var("HOME").unwrap();

        let mut abs = AbsPath::try_from("~/./hoge").unwrap();
        let abs = abs.join("fuga").unwrap();
        assert_eq!(format!("{}/./hoge/fuga", home), abs.to_string());

        let mut abs = AbsPath::try_from("~/./hoge").unwrap();
        let abs = abs.join("/fuga").unwrap();
        assert_eq!(format!("{}/./hoge/fuga", home), abs.to_string());
    }
}

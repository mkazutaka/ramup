use anyhow::{Context, Result};
use path_abs::PathInfo;
use serde::export::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AbsPath {
    path: String,
}

impl AbsPath {
    #[allow(dead_code)]
    pub fn new<P: AsRef<Path>>(p: P) -> Result<Self> {
        let p = AbsPath::expand(p).expect("|| Failed to expand Path");

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
        let path = String::from(path.as_ref().to_string_lossy());
        let path =
            AbsPath::expand(&path).with_context(|| format!("Failed to expand Path: {}", path))?;

        let path = if path.has_root() {
            let path = path.strip_prefix("/")?;
            Path::new(&self.path).join(path)
        } else {
            Path::new(&self.path).join(path)
        };

        Ok(AbsPath {
            path: String::from(path.to_string_lossy()),
        })
    }

    fn expand<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let p = String::from(path.as_ref().to_string_lossy());
        let p = p.replace("/./", "/");
        let p = Path::new(&p);

        if p.starts_with("..") {
            let current = std::env::current_dir().with_context(|| "Failed to get Current Dir")?;
            let parent = current
                .parent()
                .with_context(|| "Failed to get Current Dir")?;
            let p = p
                .strip_prefix("..")
                .with_context(|| "Failed to strip `..` from path")?;
            return Ok(parent.join(p));
        };

        if p.starts_with(".") {
            let current = std::env::current_dir().with_context(|| "Failed to get Current Dir")?;
            let p = p
                .strip_prefix(".")
                .with_context(|| "Failed to strip `.` from path")?;
            return Ok(current.join(p));
        };

        if p.starts_with("~") {
            let home = std::env::var("HOME").with_context(|| "Failed to get home Dir")?;
            let p = p
                .strip_prefix("~")
                .with_context(|| "Failed to strip `~` from path")?;
            return Ok(Path::new(&home).join(p));
        };

        Ok(p.to_path_buf())
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
    fn from() {
        let home = env::var("HOME").unwrap();

        let abs = AbsPath::try_from("~/./hoge").unwrap();
        assert_eq!(format!("{}/hoge", home), abs.to_string());
        let abs = AbsPath::try_from(Path::new("~/./hoge")).unwrap();
        assert_eq!(format!("{}/hoge", home), abs.to_string());
        let abs = AbsPath::try_from(&String::from("~/./hoge")).unwrap();
        assert_eq!(format!("{}/hoge", home), abs.to_string());
    }

    #[test]
    #[serial]
    fn parent() {
        let abs = AbsPath::try_from("~/./hoge").unwrap();
        let abs = abs.parent().unwrap();
        let home = env::var("HOME").unwrap();
        assert_eq!(format!("{}", home), abs.to_string());
    }

    #[test]
    #[serial]
    fn join() {
        let home = env::var("HOME").unwrap();

        let abs = AbsPath::try_from("~/././hoge").unwrap();
        let abs = abs.join("fuga").unwrap();
        assert_eq!(format!("{}/hoge/fuga", home), abs.to_string());

        let abs = AbsPath::try_from("~/./hoge").unwrap();
        let abs = abs.join("/fuga").unwrap();
        assert_eq!(format!("{}/hoge/fuga", home), abs.to_string());
    }

    #[test]
    #[serial]
    fn expand() {
        let home = env::var("HOME").unwrap();
        let current = env::current_dir().unwrap();
        let parent = current.parent().unwrap();

        let expect = Path::new(&current);
        let actual = AbsPath::expand(".").unwrap();
        assert_eq!(expect, actual);

        let expect = parent.to_path_buf();
        let actual = AbsPath::expand("..").unwrap();
        assert_eq!(expect, actual);

        let expect = Path::new(&current).join("hoge");
        let actual = AbsPath::expand("./hoge").unwrap();
        assert_eq!(expect, actual);

        let expect = Path::new(&home).join("hoge");
        let actual = AbsPath::expand("~/hoge").unwrap();
        assert_eq!(expect, actual);
    }
}

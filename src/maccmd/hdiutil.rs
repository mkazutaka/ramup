use crate::error::{AppError, Result};
use std::process::Command;

pub struct HdiUtil {
    pub mount_point: String,
}

impl Default for HdiUtil {
    fn default() -> Self {
        HdiUtil {
            mount_point: "".into(),
        }
    }
}

impl HdiUtil {
    pub fn attach(&mut self, image: &str) -> Result<()> {
        let output = Command::new("hdiutil")
            .args(&["attach", "-nomount", image])
            .output()?;

        if !output.status.success() {
            let err = format!("{}", String::from_utf8(output.stderr).unwrap());
            return Err(AppError::CommandError(err));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        self.mount_point = output.trim().to_string();
        Ok(())
    }

    pub fn detach(&mut self) -> Result<()> {
        let output = Command::new("hdiutil")
            .args(&["detach", &self.mount_point])
            .output()?;

        if !output.status.success() {
            let err = format!("{}", String::from_utf8(output.stderr).unwrap());
            return Err(AppError::CommandError(err));
        }
        self.mount_point = "".to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn default() {
        let hdiutl = HdiUtil::default();
        assert_eq!(hdiutl.mount_point, "".to_string())
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn attach_and_detach() {
        let mut hdiutl = HdiUtil::default();
        hdiutl.attach("ram://100").unwrap();
        assert!(hdiutl.mount_point.len() > 1);

        hdiutl.detach().unwrap();
        assert_eq!(hdiutl.mount_point.len(), 0);
    }
}

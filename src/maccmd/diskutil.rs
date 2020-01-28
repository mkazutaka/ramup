use crate::error::{AppError, Result};
use std::process::Command;

pub struct DiskUtil {
    volume_name: String,
}

impl Default for DiskUtil {
    fn default() -> Self {
        DiskUtil {
            volume_name: "RAMDisk".into(),
        }
    }
}

impl DiskUtil {
    pub fn erasevolume(&self, mount_point: &str) -> Result<()> {
        let output = Command::new("diskutil")
            .args(&["erasevolume", "HFS+", &self.volume_name, mount_point])
            .output()?;

        if !output.status.success() {
            let err = format!("{}", String::from_utf8(output.stderr).unwrap());
            return Err(AppError::CommandError(err));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maccmd::HdiUtil;

    #[test]
    #[cfg(target_os = "macos")]
    fn default() {
        let hdiutl = DiskUtil::default();
        assert_eq!(hdiutl.volume_name, "RAMDisk".to_string())
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn erasevolume() {
        let mut hdiutl = HdiUtil::default();
        hdiutl.attach("ram://100000").unwrap();
        assert!(hdiutl.mount_point.len() > 1);

        let diskutil = DiskUtil::default();
        diskutil.erasevolume(&hdiutl.mount_point).unwrap();

        hdiutl.detach().unwrap();
        assert_eq!(hdiutl.mount_point.len(), 0);
    }
}

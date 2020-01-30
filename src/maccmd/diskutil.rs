use crate::error::{AppError, Result};
use std::process::Command;

pub struct DiskUtil {}

impl DiskUtil {
    #[allow(dead_code)]
    pub fn erasevolume(name: &str, mount_point: &str) -> Result<()> {
        let output = Command::new("diskutil")
            .args(&["erasevolume", "HFS+", name, mount_point])
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
    fn erasevolume() {
        let name = "RAMDiskForTest";
        let mount_point = HdiUtil::attach(&100000).unwrap();

        DiskUtil::erasevolume(&name, &mount_point).unwrap();
        let devname = format!("{}{}", "/Volumes/", name);
        HdiUtil::detach(&devname).unwrap();
    }
}

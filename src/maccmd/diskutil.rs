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
            return Err(AppError::CommandError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "macos")]
    fn erasevolume() {
        use crate::maccmd::DiskUtil;
        use crate::maccmd::HdiUtil;

        let name = "RAMDiskForTest";
        let mount_point = HdiUtil::attach(100000).unwrap();

        DiskUtil::erasevolume(&name, &mount_point).unwrap();
        let devname = format!("{}{}", "/Volumes/", name);
        HdiUtil::detach(&devname).unwrap();
    }
}

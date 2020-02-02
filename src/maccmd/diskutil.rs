use anyhow::Result;
use std::process::Command;

pub struct DiskUtil {}

impl DiskUtil {
    #[allow(dead_code)]
    pub fn erasevolume(name: &str, mount_point: &str) -> Result<()> {
        let output = Command::new("diskutil")
            .args(&["erasevolume", "HFS+", name, mount_point])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("failed to diskutil command: {:?}", output.stderr);
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maccmd::DiskUtil;
    use crate::maccmd::HdiUtil;

    #[test]
    #[cfg(target_os = "macos")]
    fn erasevolume() {
        let name = "RAMDiskForTest";
        let mount_point = HdiUtil::attach(100000).unwrap();

        DiskUtil::erasevolume(&name, &mount_point).unwrap();
        let devname = format!("{}{}", "/Volumes/", name);
        HdiUtil::detach(&devname).unwrap();
    }
}

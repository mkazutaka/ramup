use crate::error::{AppError, Result};
use plist;
use serde::Deserialize;
use std::process::Command;

pub struct HdiUtil {}

impl HdiUtil {
    #[allow(dead_code)]
    pub fn info() -> Result<HdiUtilInfo> {
        let output = Command::new("hdiutil").args(&["info", "-plist"]).output()?;

        if !output.status.success() {
            return Err(AppError::CommandError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        };

        let output = output.stdout.to_vec();

        Ok(plist::from_bytes(&output).unwrap())
    }

    #[allow(dead_code)]
    pub fn exist(devname: &str) -> Result<bool> {
        let info = HdiUtil::info()?;
        for image in &info.images {
            if devname == image.system_entities[0].dev_entry {
                return Ok(true);
            }
            if devname == image.system_entities[0].mount_point {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[allow(dead_code)]
    pub fn exist_volume(name: &str) -> Result<bool> {
        let volume_name = format!("/Volumes/{}", name);
        HdiUtil::exist(&volume_name)
    }

    #[allow(dead_code)]
    pub fn attach(size: isize) -> Result<String> {
        let image = format!("ram://{}", size);
        let image = image.as_str();
        let output = Command::new("hdiutil")
            .args(&["attach", "-nomount", image])
            .output()?;

        if !output.status.success() {
            return Err(AppError::CommandError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        Ok(output.trim().to_string())
    }

    #[allow(dead_code)]
    pub fn detach(mountpoint: &str) -> Result<()> {
        let output = Command::new("hdiutil")
            .args(&["detach", "-force", mountpoint])
            .output()?;

        if !output.status.success() {
            return Err(AppError::CommandError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn detach_volume(name: &str) -> Result<()> {
        let volume_name = format!("/Volumes/{}", name);
        HdiUtil::detach(&volume_name)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct HdiUtilInfo {
    framework: String,
    revision: String,
    images: Vec<HdiUtilInfoImage>,
    vendor: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct HdiUtilInfoImage {
    autodiskmount: bool,
    blockcount: isize,
    blocksize: isize,
    #[serde(rename(deserialize = "hdid-pid"))]
    hdid_pid: isize,
    #[serde(rename(deserialize = "icon-path"))]
    icon_path: String,
    #[serde(rename(deserialize = "image-encrypted"))]
    image_encrypted: bool,
    #[serde(rename(deserialize = "image-path"))]
    image_path: String,
    #[serde(rename(deserialize = "image-type"))]
    image_type: String,
    #[serde(rename(deserialize = "owner-uid"))]
    owner_uid: isize,
    removable: bool,
    #[serde(rename(deserialize = "system-entities"))]
    system_entities: Vec<HdiUtilInfoImageSystemEntity>,
    writeable: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Default)]
pub struct HdiUtilInfoImageSystemEntity {
    #[serde(rename(deserialize = "content-hint"))]
    content_hint: String,
    #[serde(rename(deserialize = "dev-entry"))]
    dev_entry: String,
    #[serde(rename(deserialize = "mount-point"), default)]
    mount_point: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn info() {
        let hdiutil_info = HdiUtil::info().unwrap();
        assert_eq!(hdiutil_info.vendor, "Apple");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn exist() {
        use crate::maccmd::DiskUtil;

        let mountpoint = HdiUtil::attach(100).unwrap();
        assert_eq!(HdiUtil::exist(&mountpoint).unwrap(), true);
        HdiUtil::detach(&mountpoint).unwrap();

        let mountpoint = HdiUtil::attach(10000).unwrap();
        let name = "RAMDiskForExistTest";
        DiskUtil::erasevolume(&name, &mountpoint).unwrap();
        assert_eq!(HdiUtil::exist_volume(&name).unwrap(), true);
        HdiUtil::detach_volume(&name).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn not_exist() {
        assert_eq!(HdiUtil::exist("DevNameForNotExist").unwrap(), false);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn attach_and_detach() {
        let mountpoint = HdiUtil::attach(100).unwrap();
        assert!(mountpoint.len() > 1);
        HdiUtil::detach(&mountpoint).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn attach_and_detach_volume() {
        use crate::maccmd::DiskUtil;

        let mountpoint = HdiUtil::attach(10000).unwrap();
        let name = "RAMDiskForAttachAndDetachTest";
        DiskUtil::erasevolume(name, &mountpoint).unwrap();
        assert_eq!(HdiUtil::exist_volume(&name).unwrap(), true);
        HdiUtil::detach_volume(name).unwrap();
    }

    #[test]
    fn parse_info() {
        let info = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                    <key>framework</key>
                    <string>480.60.1</string>
                    <key>images</key>
                    <array/>
                    <key>revision</key>
                    <string>10.13v480.60.1</string>
                    <key>vendor</key>
                    <string>Apple</string>
            </dict>
            </plist>
        "#.as_bytes();
        let hdiutil_info: HdiUtilInfo = plist::from_bytes(info).expect("failed to read plist");

        assert_eq!(hdiutil_info.framework, "480.60.1");
        assert_eq!(hdiutil_info.revision, "10.13v480.60.1");
        assert_eq!(hdiutil_info.vendor, "Apple");
        assert_eq!(hdiutil_info.images.len(), 0);
    }

    #[test]
    fn parse_info_with_images() {
        let info = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                    <key>framework</key>
                    <string>480.60.1</string>
                    <key>images</key>
                    <array>
                            <dict>
                                    <key>autodiskmount</key>
                                    <false/>
                                    <key>blockcount</key>
                                    <integer>4194304</integer>
                                    <key>blocksize</key>
                                    <integer>512</integer>
                                    <key>hdid-pid</key>
                                    <integer>73094</integer>
                                    <key>icon-path</key>
                                    <string>/System/Library/PrivateFrameworks/DiskImages.framework/Resources/CDiskImage.icns</string>
                                    <key>image-encrypted</key>
                                    <false/>
                                    <key>image-path</key>
                                    <string>ram://4194304</string>
                                    <key>image-type</key>
                                    <string>read/write</string>
                                    <key>owner-uid</key>
                                    <integer>501</integer>
                                    <key>removable</key>
                                    <true/>
                                    <key>system-entities</key>
                                    <array>
                                            <dict>
                                                    <key>content-hint</key>
                                                    <string></string>
                                                    <key>dev-entry</key>
                                                    <string>/dev/disk2</string>
                                            </dict>
                                    </array>
                                    <key>writeable</key>
                                    <true/>
                            </dict>
                    </array>
                    <key>revision</key>
                    <string>10.13v480.60.1</string>
                    <key>vendor</key>
                    <string>Apple</string>
            </dict>
            </plist>
        "#.as_bytes();

        let hdiutil_info: HdiUtilInfo = plist::from_bytes(info).expect("failed to read plist");
        assert_eq!(hdiutil_info.framework, "480.60.1");
        assert_eq!(hdiutil_info.revision, "10.13v480.60.1");
        assert_eq!(hdiutil_info.vendor, "Apple");
        assert_eq!(hdiutil_info.images.len(), 1);

        let hdiuti_info_image = &hdiutil_info.images[0];
        assert_eq!(hdiuti_info_image.autodiskmount, false);
        assert_eq!(hdiuti_info_image.blockcount, 4194304);
        assert_eq!(hdiuti_info_image.blocksize, 512);
        assert_eq!(hdiuti_info_image.hdid_pid, 73094);
        assert_eq!(
            hdiuti_info_image.icon_path,
            "/System/Library/PrivateFrameworks/DiskImages.framework/Resources/CDiskImage.icns"
                .to_string()
        );
        assert_eq!(hdiuti_info_image.image_encrypted, false);
        assert_eq!(hdiuti_info_image.image_path, "ram://4194304".to_string());
        assert_eq!(hdiuti_info_image.image_type, "read/write".to_string());
        assert_eq!(hdiuti_info_image.owner_uid, 501);
        assert_eq!(hdiuti_info_image.removable, true);
        assert_eq!(hdiuti_info_image.writeable, true);

        let hdiuti_info_image_system_entry = &hdiuti_info_image.system_entities[0];
        assert_eq!(hdiuti_info_image_system_entry.content_hint, "".to_string());
        assert_eq!(
            hdiuti_info_image_system_entry.dev_entry,
            "/dev/disk2".to_string()
        );
        assert_eq!(hdiuti_info_image_system_entry.mount_point, "".to_string());
    }
}

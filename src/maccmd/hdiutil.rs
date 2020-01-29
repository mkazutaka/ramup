use crate::error::{AppError, Result};
use plist;
use serde::Deserialize;
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
    pub fn info(&self) -> Result<HdiUtilInfo> {
        let output = Command::new("hdiutil").args(&["info", "-plist"]).output()?;

        if !output.status.success() {
            let err = format!("{}", String::from_utf8(output.stderr).unwrap());
            return Err(AppError::CommandError(err));
        };

        let output = output.stdout.to_vec();

        Ok(plist::from_bytes(&output).unwrap())
    }

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

#[derive(Deserialize)]
pub struct HdiUtilInfo {
    framework: String,
    revision: String,
    images: Vec<HdiUtilInfoImage>,
    vendor: String,
}

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

#[derive(Deserialize)]
pub struct HdiUtilInfoImageSystemEntity {
    #[serde(rename(deserialize = "content-hint"))]
    content_hint: String,
    #[serde(rename(deserialize = "dev-entry"))]
    dev_entry: String,
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
    fn info() {
        let hdiutl = HdiUtil::default();
        let hdiutil_info = hdiutl.info().unwrap();

        assert_eq!(hdiutil_info.vendor, "Apple");
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
    }
}

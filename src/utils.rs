use clap::Result;
use std::process::Command;

/// diskutil erasevolume HFS+ RAMUp `hdiutil attach -nomount ram://4194304`
pub fn mount(size: &isize, volume: &str) -> Result<String> {
    let image = format!("ram://{}", size);
    let mount_point = Command::new("hdiutil")
        .args(&["attach", "-nomount", &image])
        .output()?;

    let mount_point = String::from_utf8(mount_point.stdout).unwrap();
    let mount_point = mount_point.trim();

    Command::new("diskutil")
        .args(&["erasevolume", "HFS+", volume, mount_point])
        .output()?;

    Ok(mount_point.into())
}

use anyhow::{Context, Result};
use console::Emoji;
use fs_extra::dir::CopyOptions;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn relocate<S: AsRef<Path>, P: AsRef<Path>>(from: &S, to: &P) -> Result<()> {
    let mut option = CopyOptions::new();
    option.overwrite = true;

    let from_str = from.as_ref().to_str().expect("failed to convert str");

    let size = fs_extra::dir::get_size(from)?;
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{wide_msg:.bold.dim} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    pb.set_message(&from.as_ref().to_str().expect("failed to convert str"));

    let handler = |process_info: fs_extra::TransitProcess| {
        pb.set_position(process_info.copied_bytes);
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };

    let to_dir = to.as_ref().parent().with_context(|| "No parent path")?;
    fs_extra::move_items_with_progress(&vec![from], to_dir, &option, handler)?;

    pb.finish_and_clear();
    println!("{} {} is moved.", SPARKLE, from_str);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempdir::TempDir;

    #[test]
    #[serial]
    fn relocate_file() {
        let from_dir = TempDir::new("ramup").unwrap();
        let from_file = from_dir.path().join("mo ved.txt");

        std::fs::File::create(&from_file).unwrap();

        let to_dir = TempDir::new("ramup").unwrap();
        let to_file = to_dir.path().join("mo ved.txt");

        assert_eq!(false, to_file.exists());
        relocate(&from_file, &to_file).unwrap();
        assert_eq!(true, to_file.exists());
    }

    #[test]
    #[serial]
    fn relocate_dir() {
        let from_dir = TempDir::new("ramup").unwrap();
        let from = from_dir.path().join("from");
        std::fs::create_dir(&from).unwrap();

        let to_dir = TempDir::new("ramup").unwrap();
        let to = to_dir.path().join("from");

        assert_eq!(false, to.exists());
        relocate(&from, &to).unwrap();
        assert_eq!(true, to.exists());
    }
}

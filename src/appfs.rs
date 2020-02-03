use anyhow::{Context, Result};
use fs_extra::dir::{CopyOptions, TransitProcess};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub fn relocate<S: AsRef<Path>, P: AsRef<Path>>(from: &S, to: &P) -> Result<()> {
    let mut option = CopyOptions::new();
    option.overwrite = true;

    let size = fs_extra::dir::get_size(from)?;
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    let handler = |process_info: TransitProcess| {
        pb.set_position(process_info.copied_bytes);
        fs_extra::dir::TransitProcessResult::ContinueOrAbort
    };

    let to_dir = to.as_ref().parent().with_context(|| "No parent path")?;
    fs_extra::dir::move_dir_with_progress(from, to_dir, &option, handler)?;
    Ok(())
}

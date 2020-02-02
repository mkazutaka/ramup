use crate::application::Application;
use crate::env;
use crate::maccmd::{DiskUtil, HdiUtil};
use crate::path::AbsPath;
use crate::ram::RAM;
use crate::state::State;
use anyhow::{Context, Result};
use fs_extra::dir::CopyOptions;
use std::convert::TryFrom;
use std::path::Path;

pub struct Handler {
    ram: RAM,
    apps: Vec<Application>,
    state: State,
}

impl Handler {
    pub fn new(ram: RAM, apps: Vec<Application>, state: State) -> Self {
        Handler { ram, apps, state }
    }

    pub fn backup_all(&mut self) -> Result<()> {
        Handler::mount(&self.ram)?;
        Handler::_backup_all(&self.ram, &self.apps, &mut self.state)
    }

    fn _backup_all(ram: &RAM, apps: &Vec<Application>, state: &mut State) -> Result<()> {
        for app in apps {
            for path in &app.paths {
                Handler::_backup(path, ram, state)?
            }
        }
        Ok(())
    }

    pub fn backup<P: AsRef<Path>>(&mut self, s_path: P) -> Result<()> {
        Handler::mount(&self.ram)?;
        Handler::_backup(s_path, &self.ram, &mut self.state)
    }

    fn _backup<P: AsRef<Path>>(s_path: P, ram: &RAM, state: &mut State) -> Result<()> {
        let t_path = AbsPath::try_from(&ram.mount_path)?
            .join(&ram.name)?
            .join(&s_path)?;
        let t_dir = t_path.parent()?;

        std::fs::create_dir_all(&t_dir)?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(&s_path, &t_dir, &option)?;
        std::os::unix::fs::symlink(&t_path, &s_path)?;

        state.add_and_save(&s_path)?;
        Ok(())
    }

    pub fn restore_all(&mut self) -> Result<()> {
        Handler::_restore_all(&self.ram, &self.apps, &mut self.state);
        self.clean()
    }

    fn _restore_all(ram: &RAM, apps: &Vec<Application>, state: &mut State) -> Result<()> {
        for app in apps {
            for path in &app.paths {
                Handler::_restore(path, ram, state)?
            }
        }
        Ok(())
    }

    pub fn restore<P: AsRef<Path>>(&mut self, t_path: P) -> Result<()> {
        Handler::_restore(t_path, &self.ram, &mut self.state);
        Ok(())
    }

    fn _restore<P: AsRef<Path>>(t_path: P, ram: &RAM, state: &mut State) -> anyhow::Result<()> {
        let t_path: &Path = t_path.as_ref();
        let t_dir = t_path.parent().with_context(|| "There isn't parent path")?;

        let s_path = AbsPath::try_from(&ram.mount_path)?
            .join(&ram.name)?
            .join(&t_path)?;

        std::fs::remove_file(t_path).with_context(|| "Cannot Delete file")?;
        let mut option = CopyOptions::new();
        option.copy_inside = true;
        fs_extra::dir::move_dir(&s_path, t_dir, &option)?;

        state.remove_and_save(&t_path);
        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        std::fs::remove_file(env::get_state_path())?;
        Handler::unmount(&self.ram)
    }

    fn mount(ram: &RAM) -> Result<()> {
        if HdiUtil::exist_volume(&ram.name)? {
            return Ok(());
        }
        let mountpoint = HdiUtil::attach(ram.size)?;
        DiskUtil::erasevolume(&ram.name, &mountpoint)
    }

    fn unmount(ram: &RAM) -> Result<()> {
        if !HdiUtil::exist_volume(&ram.name)? {
            return Ok(());
        }
        HdiUtil::detach_volume(&ram.name)
    }
}

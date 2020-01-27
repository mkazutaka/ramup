use crate::config::Config;
use crate::utils;
use clap::Result;
use fs_extra::dir::{move_dir, CopyOptions};
use shellexpand;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Ramup {
    config: Config,
    mount_point: String,
}

impl Ramup {
    pub fn new(config: Config) -> Ramup {
        Ramup {
            config,
            mount_point: "".to_string(),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        self.mount_point = utils::mount(&self.config.ram.size, &self.config.ram.name)?;
        Ok(())
    }

    pub fn backup(&mut self) {
        for app in &mut self.config.applications {
            app.backup(&self.config.ram.name);
        }
    }

    pub fn restore(&self) {
        for app in &self.config.applications {
            app.restore(&self.config.ram.name);
        }
    }
}

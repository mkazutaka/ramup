mod application;
mod config;
mod env;
mod error;
mod maccmd;
mod path;
mod ram;
mod ramup;
mod state;

use crate::ramup::Ramup;
use anyhow::Result;
use clap::load_yaml;
use clap::{App, Arg, ArgMatches, SubCommand};
use shellexpand;
use std::path::Path;

static SUB_COMMAND_INIT: &str = "init";
static SUB_COMMAND_BACKUP: &str = "backup";
static SUB_COMMAND_RESTORE: &str = "restore";
static SUB_COMMAND_CLEAN: &str = "clean";

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let arg_matches = App::from_yaml(yaml).get_matches();

    let cfg_path = shellexpand::tilde("~/.config/ramup/config.toml").to_string();

    if arg_matches.subcommand_matches(SUB_COMMAND_INIT).is_some() {
        let cfg_path = std::path::Path::new(&cfg_path);
        if cfg_path.exists() {
            return Ok(());
        }
        crate::config::Config::initialize().unwrap();
        return Ok(());
    }

    let ramup = Ramup::from_file()?;

    if arg_matches.subcommand_matches(SUB_COMMAND_BACKUP).is_some() {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.backup(path)?;
            }
        } else {
            ramup.backup_all()?;
        }
        return Ok(());
    }

    if arg_matches
        .subcommand_matches(SUB_COMMAND_RESTORE)
        .is_some()
    {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.restore(path).unwrap();
            }
        } else {
            ramup.restore_all().unwrap();
        }
    }

    if arg_matches.subcommand_matches(SUB_COMMAND_CLEAN).is_some() {
        ramup.clean();
    }

    Ok(())
}

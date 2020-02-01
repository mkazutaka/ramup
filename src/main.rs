mod application;
mod config;
mod env;
mod error;
mod maccmd;
mod ram;
mod ramup;
mod state;

use crate::ramup::Ramup;
use anyhow::Result;
use clap::{App, Arg, SubCommand};
use shellexpand;

fn main() -> Result<()> {
    let matches = App::new("ramup")
        .version("v0.1.0")
        .author("mkazutaka <paper.sheet.kami@gmail.com")
        .subcommand(
            SubCommand::with_name("init")
                .author("initialize ramup's config")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Target path to backup"),
                ),
        )
        .subcommand(
            SubCommand::with_name("backup")
                .author("backup to RAM Disk")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Target path to backup"),
                ),
        )
        .subcommand(
            SubCommand::with_name("restore")
                .author("restore from RAM Disk")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Target path to backup"),
                ),
        )
        .subcommand(SubCommand::with_name("clean").author("clean RAM Disk"))
        .get_matches();

    let cfg_path = shellexpand::tilde("~/.config/ramup/config.toml").to_string();

    if matches.subcommand_matches("init").is_some() {
        let cfg_path = std::path::Path::new(&cfg_path);
        if cfg_path.exists() {
            return Ok(());
        }
        crate::config::Config::initialize().unwrap();
        return Ok(());
    }

    let ramup = Ramup::from_file().unwrap();

    if let Some(matches) = matches.subcommand_matches("backup") {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.backup(path).unwrap();
            }
        } else {
            ramup.backup_all().unwrap();
        }
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("restore") {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.restore(path).unwrap();
            }
        } else {
            ramup.restore_all().unwrap();
        }
    }

    if matches.subcommand_matches("clean").is_some() {
        ramup.clean().unwrap();
        return Ok(());
    }

    Ok(())
}

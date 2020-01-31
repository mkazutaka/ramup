mod application;
mod config;
mod error;
mod maccmd;
mod ram;
mod ramup;
mod state;

use crate::ramup::Ramup;
use clap::{App, Arg, SubCommand};
use shellexpand;

fn main() {
    let matches = App::new("ramup")
        .version("v0.1.0")
        .author("mkazutaka <paper.sheet.kami@gmail.com")
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

    let cfg = shellexpand::tilde("~/.config/ramup/config.toml").to_string();
    let ramup = Ramup::from_file(&cfg).unwrap();

    if let Some(matches) = matches.subcommand_matches("backup") {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.backup(path).unwrap();
            }
        } else {
            ramup.backup_all().unwrap();
        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("restore") {
        if matches.is_present("path") {
            if let Some(path) = matches.value_of("path") {
                ramup.restore(path).unwrap();
            }
        } else {
            ramup.restore_all().unwrap();
        }
        return;
    }

    if matches.subcommand_matches("clean").is_some() {
        ramup.clean().unwrap();
        return;
    }
}

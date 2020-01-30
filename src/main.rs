mod application;
mod config;
mod error;
mod maccmd;
mod ram;
mod ramup;

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
                        .help("Target path to backup"),
                ),
        )
        .get_matches();

    let cfg = shellexpand::tilde("~/.config/ramup/config.toml").to_string();
    let ramup = Ramup::from_file(&cfg).unwrap();

    if let Some(matches) = matches.subcommand_matches("backup") {
        if matches.is_present("path") {
            match matches.value_of("path") {
                Some(path) => ramup.backup(path).unwrap(),
                None => {}
            }
        } else {
            ramup.backup_all().unwrap();
        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("restore") {
        if matches.is_present("path") {
            match matches.value_of("path") {
                Some(path) => ramup.restore(path).unwrap(),
                None => {}
            }
        } else {
            ramup.restore_all().unwrap();
        }
        return;
    }
}

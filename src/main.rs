mod application;
mod config;
mod error;
mod maccmd;
mod ram;
mod ramup;
mod utils;

use crate::ramup::Ramup;
use clap::{App, Arg, SubCommand};
use ctrlc;
use shellexpand;
use std::process;
use std::{thread, time::Duration};

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

    let config = config::Config::new();

    let cfg = shellexpand::tilde("~/.config/ramup/config.toml").to_string();
    let mut ramup = Ramup::from_file(&cfg);

    if let Some(matches) = matches.subcommand_matches("backup") {
        //        if matches.is_present("debug") {
        //            println!("Printing debug info...");
        //        } else {
        //            println!("Printing normally...");
        //        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("restore") {
        //        if matches.is_present("debug") {
        //            println!("Printing debug info...");
        //        } else {
        //            println!("Printing normally...");
        //        }
        return;
    }

    //    let mut ramup = Ramup::new_old(config);
    //
    //    ramup.create_old().unwrap();
    //    ramup.backup_old();
    //
    //    ctrlc::set_handler(move || {
    //        let config = config::Config::new();
    //        let mut ramup = Ramup::new_old(config);
    //        ramup.restore_old();
    //        process::exit(1)
    //    })
    //    .unwrap();
    //
    //    loop {
    //        ramup.rsync_old();
    //        thread::sleep(Duration::from_secs(5));
    //    }
}

mod applications;
mod config;
mod ramup;

use crate::ramup::Ramup;
use clap::{App, SubCommand};
use ctrlc;
use std::process;
use std::{error::Error, thread, time::Duration};

fn main() {
    let matches = App::new("ramup")
        .version("v0.1.0")
        .subcommand(SubCommand::with_name("backup").about("backup on RAM Disk"))
        .subcommand(SubCommand::with_name("restore").about("restore from RAM Disk"))
        .get_matches();

    let user_config = config::UserConfig::new();
    let mut ramup = Ramup::new(user_config);

    ramup.create();
    ramup.backup();

    ctrlc::set_handler(move || {
        ramup.restore();
        process::exit(1)
    });

    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

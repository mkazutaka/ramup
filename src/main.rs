mod application;
mod config;
mod ramup;
mod utils;

use crate::ramup::Ramup;
use clap::{App, SubCommand};
use ctrlc;
use std::process;
use std::{thread, time::Duration};

fn main() {
    let _matches = App::new("ramup")
        .version("v0.1.0")
        .subcommand(SubCommand::with_name("backup").about("backup on RAM Disk"))
        .subcommand(SubCommand::with_name("restore").about("restore from RAM Disk"))
        .get_matches();

    let config = config::Config::new();
    let mut ramup = Ramup::new(config);

    ramup.create().unwrap();
    ramup.backup();

    ctrlc::set_handler(move || {
        ramup.restore();
        process::exit(1)
    })
    .unwrap();

    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

mod appenv;
mod appfs;
mod application;
mod apppath;
mod cfg;
mod handler;
mod maccmd;
mod ram;
mod state;
mod subcmd;

use anyhow::{Context, Result};
use clap::load_yaml;
use clap::App;

//static SUB_COMMAND_INIT: &str = "init";
static SUB_COMMAND_BACKUP: &str = "backup";
static SUB_COMMAND_RESTORE: &str = "restore";
//static SUB_COMMAND_CLEAN: &str = "clean";

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let arg_matches = App::from_yaml(yaml).get_matches();

    let config = cfg::Config::load()?;
    let state = state::State::load();
    let apps = config.applications;
    let ram = config.ram;

    let mut handler = handler::Handler::new(ram, state);

    match arg_matches.subcommand_name() {
        Some("init") => cfg::Config::initialize()?,
        Some("backup") => {
            let matches = arg_matches
                .subcommand_matches(SUB_COMMAND_BACKUP)
                .with_context(|| "Arg not found")?;
            let mut sources: Vec<String> = vec![];
            if matches.is_present("path") {
                let path = matches.value_of("path").with_context(|| "path not found")?;
                sources.push(path.to_string());
            } else {
                for app in &apps {
                    for path in &app.paths {
                        sources.push(path.clone());
                    }
                }
            }
            handler.backup(sources)?
        }
        Some("restore") => {
            let matches = arg_matches
                .subcommand_matches(SUB_COMMAND_RESTORE)
                .with_context(|| "Arg not found")?;
            let mut sources: Vec<String> = vec![];
            if matches.is_present("path") {
                let path = matches.value_of("path").with_context(|| "path not found")?;
                sources.push(path.to_string());
            } else {
                for app in &apps {
                    for path in &app.paths {
                        sources.push(path.clone());
                    }
                }
            }
            handler.restore(sources)?
        }
        Some("clean") => handler.clean()?,
        _ => (),
    }

    Ok(())
}

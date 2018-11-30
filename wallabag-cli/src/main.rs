
mod backend;

use std::env;
use std::fs::File;
use std::io;

use clap::{App, Arg, SubCommand};
use log::{debug, error, info, warn};
use simplelog::WriteLogger;

use wallabag_api::types::Config;
use wallabag_api::Client;

use crate::backend::Backend;

const INIT: &'static str = "init";

fn main() -> Result<(), failure::Error> {
    // init logging
    WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("wallabag-cli.log").unwrap(),
    );

    let app = App::new("Wallabag CLI")
        .version("alpha")
        .about("Command line client for Wallabag")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Use a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name(INIT).about("init the database"));

    let matches = app.get_matches();

    let db = Backend::new("db.sqlite3");

    match matches.subcommand_name() {
        None => {
            println!("No subcommand given.");
            let res = db.load_tags();
            println!("{:#?}", res);
        }
        Some(INIT) => {
            println!("Initing the database...");
            let res = db.up();
            println!("{:#?}", res);
        }
        _ => {
            // shrug
        }
    }

    Ok(())
}

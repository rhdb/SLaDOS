#[cfg(not(unix))]
compile_error!("SLaDOS cannot run on anything but a UNIX machine, due to the authors lazyness.");

#[macro_use]
extern crate lazy_static;

pub mod quote;
pub mod kiosk;
pub mod config;
pub mod multicast;
pub mod dispatch;

use rand::prelude::SliceRandom;
use clap::{Arg, App, AppSettings};
use log::{trace, info, warn};

use config::ConfigurationFile;

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    let matches = App::new("SLaDOS")
        .version("1.0.0")
        .author("Milo Banks <milobanks@rowlandhall.org>, Eli Hatton <elihatton@rowlandhall.org>")
        .about("Standardised Logging and Decentralized student-Observation System.")
        .setting(AppSettings::SubcommandRequired)
        .arg(Arg::new("config")
            .about("Where to find the configuration file.")
            .long("config")
            .value_name("FILE")
            .default_value("/etc/slados.d/config.toml")
            .takes_value(true))
        .subcommand(App::new("kiosk")
            .about("Run as a kiosk. Pretty much the only option. Deals both with registration and attendance."))
        .get_matches();

    info!("Reading configuration file");
    let config = config::parse_config_file(PathBuf::from(matches.value_of("config").unwrap())).expect("woops");

    trace!("Configuration data: {:?}", config);

    warn!("{}", quote::QUOTES.choose(&mut rand::thread_rng()).unwrap());

    if ! config.enabled {
        info!("Stopping the entire show, because you wanted us to. Don't you apreciate our work?");
        std::process::exit(0); // TODO: Dynamically get the actual OS "ok" exit code
    }

    subcommand_handler(matches, config);
}

fn subcommand_handler(matches: clap::ArgMatches, config: ConfigurationFile) {
    match matches.subcommand() {
        Some(("kiosk", _)) => {
            kiosk::kiosk(config);
        },

        _ => unreachable!(),
    }
}

pub mod quote;
pub mod kiosk;
pub mod server;
pub mod config;

// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::prelude::SliceRandom;
use clap::{Arg, App, AppSettings};
use log::{trace, info, warn, error};

use config::ConfigurationFile;

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    let matches = App::new("SLaDOS")
        .version("1.0.0")
        .author("Milo Banks <milobanks@rowlandhall.org>, Eli Hatton <elihatton@rowlandhall.org>, General RHDB")
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
        .subcommand(App::new("server")
            .about("Run as a server. The exception for the only option thing mentioned earlier."))
        .get_matches();

    info!("Reading configuration file");
    let config = match config::parse_config_file(PathBuf::from(matches.value_of("config").unwrap())) {
        Ok(c) => c,
        // Even though we normally use a . instead of a :, serde errors are
        // not capitalised, so it wouldn't make sense to put a . here.
        Err(e) => { error!("Failed to parse the configuration file: {}", e); return }
    };

    trace!("Configuration data: {:?}", config);

    warn!("{}", quote::QUOTES.choose(&mut rand::thread_rng()).unwrap());

    if ! config.enabled {
        info!("Stopping the entire show, because you wanted us to. Don't you apreciate our work?");
        std::process::exit(0); // TODO: Dynamically get the actual OS "ok" exit code
    }

    subcommand_handler(matches, config).await;
}

async fn subcommand_handler(matches: clap::ArgMatches, config: ConfigurationFile) {
    match matches.subcommand() {
        Some(("kiosk", _)) => {
            if config.kiosk.is_none() {
                error!("Please supply a kiosk section in the configuration file!");
                return;
            }

            kiosk::kiosk(config);
        },

        Some(("server", _)) => {
            if config.server.is_none() {
                error!("Please supply a server section in the configuration file!");
                return;
            }

            server::server(config).await;
        },

        _ => unreachable!(),
    }
}

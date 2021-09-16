pub mod registration;
pub mod checkpoint;
pub mod config;
pub mod dispatch;

use clap::{Arg, App, AppSettings};
use log::{trace, info};

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
            .about("Run as a kiosk. Pretty much the only option.")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(App::new("registration")
                .about("A registration kiosk."))
            .subcommand(App::new("checkpoint")
                .about("A kiosk to keep track of attendance. A checkpoint."))
            )
        .get_matches();

    info!("Reading configuration file");
    let config = config::parse_config_file(PathBuf::from(matches.value_of("config").unwrap())).expect("woops");

    trace!("Configuration data: {:?}", config);

    if ! config.enabled {
        info!("Stopping the entire show, because you wanted us to. Don't you apreciate our work?");
        std::process::exit(0); // TODO: Dynamically get the actual OS "ok" exit code
    }

    subcommand_handler(matches);
}

fn subcommand_handler(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("kiosk", kiosk_args)) => {
            match kiosk_args.subcommand() {
                Some(("registration", _registration_args)) => {
                    info!("Starting kiosk in registration mode!");
                    registration::registration();
                },

                Some(("checkpoint", _checkpoint_args)) => {
                    info!("Starting kiosk in checkpoint mode!");
                    checkpoint::checkpoint();
                },

                _ => unreachable!(),
            }
        },

        _ => unreachable!(),
    }
}

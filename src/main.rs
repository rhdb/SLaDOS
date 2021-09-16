pub mod dispatch;

use clap::{Arg, App, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = App::new("SLaDOS")
        .version("1.0.0")
        .author("Milo Banks <milobanks@rowlandhall.org>, Eli Hatton <elihatton@rowlandhall.org>")
        .about("Standardised Logging and Decentralized student-Observation System.")
        .arg(Arg::with_name("config")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .subcommand(SubCommand::with_name("kiosk")
            .about("Run as a kiosk. Pretty much the only option.")
            .subcommand(SubCommand::with_name("registration")
                .about("A registration kiosk."))
            .subcommand(SubCommand::with_name("checkpoint")
                .about("A kiosk to keep track of attendance. A checkpoint."))
            )
        .get_matches();

    dispatch::powered_dispatch("127.0.0.1:8000".to_owned()).await
} 

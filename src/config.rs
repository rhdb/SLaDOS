use serde::Deserialize;
use log::debug;

use std::path::PathBuf;
use std::fs;

/// The struct that holds the endpoint for the deserialization of the configuration file.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationFile {
    /// Should we actually do anything?
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Should use IPv4 or IPv6
    #[serde(default = "default_ip_version")]
    pub ip_version: IpVersion,

    /// The host to bind on. Required.
    pub host: String,

    /// The port to bind on.
    #[serde(default = "default_port")]
    pub port: u16,

    /// The subset of the configuration file that
    /// deals exclusively with server configuration.
    pub server: Option<ServerConfig>,

    /// THe subset of the configuration file that
    /// deals exclusively with kiosk/client configuration.
    pub kiosk: Option<ServerConfig>,
}

/// A subset of the main configuration struct.
/// Dedicated to client configuration values.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    /// The location of the s2id database.
    #[serde(default = "default_s2id")]
    pub s2id: String,
}

/// A subset of the main configuration struct.
/// Dedicated to client configuration values.
#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientConfig { }

/// The mode of operation that SLaDOS should run in
#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub enum IpVersion {
    V4,
    V6,
}

/// The default location of the s2id database
fn default_s2id() -> String { "/var/lib/persistant_s2id.json".to_owned() }

/// The default value of enabled
fn default_enabled() -> bool { true }

/// The default IP Protocol (RAS Syndrome) version
fn default_ip_version() -> IpVersion { IpVersion::V4 }

/// The default port to use for the server
fn default_port() -> u16 { 7645 }

/// Parses the configuration file
pub fn parse_config_file(path: PathBuf) -> Result<ConfigurationFile, std::io::Error> {
    debug!("Reading file at {}", path.display());
    let src = fs::read_to_string(path)?;

    debug!("Parsing configuration file");
    let config = toml::from_str::<ConfigurationFile>(&src)?;

    Ok(config)
}




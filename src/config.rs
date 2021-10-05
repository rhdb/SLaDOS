use serde::{/* Deserializer, */ Deserialize /* , Deserializer */};
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

    // /// The host to bind on.
    // #[serde(default = "default_host")]
    // pub host: String,

    /// Assume that there is a preexisting network
    /// (e.g. don't become a lead node if there is none
    /// existing)
    #[serde(default = "default_assume_preexisting")]
    pub assume_preexisting: bool,

    /// Assume we should just assume the host duty
    #[serde(default = "default_assume_host")]
    pub default_as_host: bool,

    /// If we should scan for a lead node, or
    /// just use the built in option.
    #[serde(default = "default_builtin_lead")]
    pub buildin_lead: Option<String>,

    /// Should use IPv4 or IPv6
    #[serde(default = "default_ip_version")]
    pub ip_version: IpVersion,

    /// How often to send a lead discovery packet, in seconds
    #[serde(default = "default_discovery_send_wait")]
    pub discovery_send_wait: u64,
}

/// The mode of operation that SLaDOS should run in
#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub enum IpVersion {
    V4,
    V6,
}

/// The default value of enabled
fn default_enabled() -> bool { true }

/// The default value for default_as_host
fn default_assume_host() -> bool { false }

/// The default value for the assume preexinsting value
fn default_assume_preexisting() -> bool { false }

/// The default builtin lead node
fn default_builtin_lead() -> Option<String> { None }

/// The default IP Protocol (RAS Syndrome) version
fn default_ip_version() -> IpVersion { IpVersion::V4 }

/// The default time to wait between sending discovery packets
fn default_discovery_send_wait() -> u64 { 60 }

/* fn deserialize_operation_mode<D>(de: &mut D) -> Result<OperationMode, D::Error>
    where D: Deserializer
{
    let s: String = Deserialize::deserialize(de)?;
    match s.as_ref() {
        "registration" => Ok(OperationMode::Registration),
        "checkpoint" => Ok(OperationMode::Checkpoint),
        other => Err(serde::de::Error::custom(format!("unknown operation mode: {}", other))),
    }
} */

/// Parses the configuration file
pub fn parse_config_file(path: PathBuf) -> Result<ConfigurationFile, std::io::Error> {
    debug!("Reading file at {}", path.display());
    let src = fs::read_to_string(path)?;

    debug!("Parsing configuration file");
    let config = toml::from_str::<ConfigurationFile>(&src)?;

    Ok(config)
}




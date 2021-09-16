use serde::{/* Deserializer, */ Deserialize /* , Deserializer */};
use log::debug;

use std::path::PathBuf;
use std::fs;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationFile {
    /// Should we actually do anything?
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// The host to bind on.
    #[serde(default = "default_host")]
    pub host: String,

    /// If we should scan for a lead node, or
    /// just use the built in option.
    #[serde(default = "default_builtin_lead")]
    pub buildin_lead: Option<String>,

    /// The mode to run in. If left empty, then
    /// we require a subcommand after kiosk. If
    /// not, then we can still take a subcommand
    /// after kiosk, and have it override this
    /// setting.
    #[serde(default = "default_operation_mode")]
    pub mode: Option<OperationMode>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub enum OperationMode {
    Registration,
    Checkpoint,
}

fn default_enabled() -> bool { true }
fn default_host() -> String { "127.0.0.1:9010".to_owned() } // TODO: Dynamically get the outward facing IP
fn default_builtin_lead() -> Option<String> { None }
fn default_operation_mode() -> Option<OperationMode> { None }

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

pub fn parse_config_file(path: PathBuf) -> Result<ConfigurationFile, std::io::Error> {
    debug!("Reading file at {}", path.display());
    let src = fs::read_to_string(path)?;

    debug!("Parsing configuration file");
    let config = toml::from_str::<ConfigurationFile>(&src)?;

    Ok(config)
}

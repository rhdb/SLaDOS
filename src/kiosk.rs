use super::config::ConfigurationFile;
use super::multicast;

pub fn kiosk(config: ConfigurationFile) {
    // First, listen for a lead node (on multicast)
    multicast::multicast_listener(config.ip_version);
}


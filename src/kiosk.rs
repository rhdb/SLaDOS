use super::config::ConfigurationFile;
use super::multicast;

pub fn kiosk(config: ConfigurationFile) {
    // First, listen for a lead node (on multicast)
    let multicast_listener_thread = multicast::multicast_listener(config.ip_version);

    multicast_listener_thread.join().unwrap();
}


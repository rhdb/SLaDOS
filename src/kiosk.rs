use super::config::ConfigurationFile;
use super::multicast;

use log::{debug, error};

pub fn kiosk(config: ConfigurationFile) {
    if ! config.default_as_host {
        debug!("Not defaulting as host...");

        // First, listen for a lead node (on multicast)
        let multicast_listener_thread = multicast::multicast_listener(config.ip_version);

        match multicast_listener_thread.join() {
            Ok(()) => (),
            Err(e) => { error!("Multicast listener thread failed. {:?}", e); return; }
        };

        return;
    }

    debug!("Defaulting as host...");

    let sender = match multicast::new_sender(config.ip_version) {
        Ok(s) => s,
        Err(e) => { error!("Failed to create new multicast sender socket. {}", e); return; },
    };

    let multicast_sender_socket = sender.0;
    let multicast_sender_addr = sender.1;

    multicast_sender_socket.send_to(b"Hello, world!", &multicast_sender_addr).expect("testing error");

    debug!("Sending test message (not production code, for testing)");

}


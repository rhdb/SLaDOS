use super::config::ConfigurationFile;
use super::multicast;

use uuid::Uuid;
use log::{debug, info, error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct DiscoveryMessage {
    uuid: u128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct DiscoveryResponce {

}

impl DiscoveryMessage {
    pub fn new(uuid: u128) -> Self {
        Self { uuid }
    }
}

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
    host(config);
}

fn host(config: ConfigurationFile) {
    let sender = match multicast::new_sender(config.ip_version) {
        Ok(s) => s,
        Err(e) => { error!("Failed to create new multicast sender socket. {}", e); return; },
    };

    let multicast_sender_socket = sender.0;
    let multicast_sender_addr = sender.1;

    info!("Joining multicast sending stuff at {}", multicast_sender_addr);

    // In the off chance that another node is on the network, this uuid
    // should be used by other nodes to differenciate. Because of this,
    // the different leads should not actually interfere with each other.
    // It is up to the administrator to determin the problem, and resolve
    // it.
    let lead_uuid = Uuid::new_v4();

    let discovery_packet = bincode::serialize(&DiscoveryMessage::new(lead_uuid.to_u128_le())).expect("Bincode serialization failed for some reason or other.");

    info!("Starting discovery packet loop (sending message every {} seconds)!", config.discovery_send_wait);
    loop {
        match multicast_sender_socket.send_to(&discovery_packet.to_vec(), &multicast_sender_addr) {
            Ok(_) => debug!("Discovery message sent."),
            Err(e) => error!("Failed to send multicast message (for lead discovery). {}", e),
        };

        std::thread::sleep(std::time::Duration::from_secs(config.discovery_send_wait));
    }

}


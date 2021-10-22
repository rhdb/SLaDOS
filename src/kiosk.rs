use super::nfc;
use super::config::ConfigurationFile;

use serde::{Serialize, Deserialize};
use hyper::Client;
use log::{info, error};

const REQUEST_ERROR: u8 = 0;
const CLIENT_ERROR: u8 = 0;
const SERVER_ERROR: u8 = 0;

#[derive(Debug, Deserialize)]
pub struct IdResponse {
    id: u32,
}

#[derive(Debug, Serialize)]
pub struct SerialToIdSendof {
    id: u32,
    serial: u32,
}

/// Is the kiosk driver.
pub async fn kiosk(config: ConfigurationFile) {
    // Establish contact with the s2id server
    let client = Client::new();
    let resource = "http://".to_owned() + &config.host +
        ":" + &config.port.to_string();

    loop {
        // Read from the NFC reader. NOTE: Eli
        let serial = nfc::read_nfc();

        info!("Read serial {}.", serial);

        // Contact the s2id server
        let uri: hyper::Uri = (resource.clone() + format!("/s2id?serial={}", serial).as_str()).as_str()
            .parse::<hyper::Uri>().expect("Unreachable URI resource+request parse error.");
        
        let response = match client.get(uri).await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to request data from the s2id server. {}", e);
                handle_error(REQUEST_ERROR);
                continue;
            },
        };

        info!("Made request, success (not yet for status).");

        // Make sure all is well
        match check_status(response.status()) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // We need to register the user
                
                register();
            },
            Err(_) => continue,
        };

        info!("Response code is well.");

        // Fet the body
        let body = get_body_from_response(response).await;

        // Data should always be serialized correctly by s2id.
        let parsed: IdResponse = serde_json::from_str(&body).expect("Invalid JSON data received");
        let id = parsed.id;

        info!("Recieved student ID: {}", id);
        
        // Mutate the database based on the ID.

    }
}

/// Gets a response from a parsed URI.
async fn get_body_from_response(response: hyper::Response<hyper::Body>) -> String {
    // Acutally receive the body
    let response_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    String::from_utf8(response_bytes.to_vec()).expect("Unreachable non-utf8 response")
}

/// Checks if a reponse is ok
fn check_status(status: hyper::StatusCode) -> Result<(), std::io::Error> {
    if status == 418 {
        // User not registered
        
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    } else if status.is_client_error() {
        error!("Client error. {}", status);
        handle_error(CLIENT_ERROR);
        
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    } else if status.is_server_error() {
        error!("Server error. {}", status);
        handle_error(SERVER_ERROR);
        
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    return Ok(());
}

/// Register a user.
fn register() {
    // NOTE: Eli, do some I/O here, like flashing a yellow light,
    // NOTE: that signifies that they need to follow the registration
    // NOTE: instructions. Thanks!
    
    let id = get_inputed_id();

}

fn get_inputed_id() {
    // NOTE: Eli, figure out a way to get the user
    // NOTE: to input their ID, to complete their
    // NOTE: registration.
}

/// Handles a kiosk error.
fn handle_error(error: u8) {
    // NOTE: Eli, do some I/O here, like flashing a red light
    // NOTE: or sounding some kind of failure sound.
    
    #[allow(unreachable_patterns)]
    match error {
        REQUEST_ERROR => {},
        CLIENT_ERROR => {},
        SERVER_ERROR => {},
        _ => unreachable!(),
    }
}


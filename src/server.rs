use super::config::ConfigurationFile;

use std::str;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{trace, info, error};

// #[tokio::main]
/// Starts the server that is *the server*, for updates
/// and syncronisation code. This does not scan for lead nodes,
/// as that happens before this function should be run.
pub async fn server(config: ConfigurationFile) {
    let _server_config = config.server.unwrap();
    let bind_on = config.host + ":" + &config.port.to_string();

    info!("Attempting to bind on {}", bind_on);
    let listener = match TcpListener::bind(&bind_on).await {
        Ok(v) => v,
        Err(e) => { error!("Failed to bind to {}. {}", bind_on, e); return },
    };

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(v) => v,
            Err(e) => { error!("Failed to accept connection. {}", e); return },
        };

        match tokio::spawn(async move {
            // In a loop, read data from the socket and write the data back.
            loop {
                trace!("Received data: {:?}", read_stream(&mut socket).await);
            }
        }).await {
            Ok(()) => (),
            Err(e) => { error!("Failed to wait on concurrent tokio centric network I/O. {}", e); return },
        }
    }
}

async fn read_stream(stream: &mut TcpStream) -> Result<(Vec<u8>, usize), std::io::Error> {
    let mut request_buffer: Vec<u8> = vec![];
        
    // Let us loop & try to read the whole request data
    let mut request_len: usize = 0;
    
    stream.poll_read_ready();

    loop {
        let mut buffer = vec![0; 1024];
       
        match stream.try_read(&mut buffer) {
            // We read successfully, n is the number of bytes read
            Ok(n) => {
                trace!("Read {} bytes", n);
                // Are we done with reading?
                if n == 0 {
                    break;
                }

                request_buffer.append(&mut buffer);
                request_len += n;
            },

            /* Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Don't do anything. Just means we have no data to read.
            }, */

            Err(e) => { error!("Couldn't read from TCP Stream. {}", e); return Err(e) },
        };
    }
    
    Ok((request_buffer, request_len))
}

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};

// #[tokio::main]
/// Starts the server that listens to the lead node, for updates
/// and syncronisation code. This does not scan for lead nodes,
/// as that happens before this function should be run.
pub async fn powered_dispatch(bind_on: String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&bind_on).await?;
    info!("Binding on {}", bind_on);


    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        error!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
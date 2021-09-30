use std::net::{UdpSocket, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::io;
use std::time::Duration;
use std::thread::JoinHandle;

use super::config::IpVersion;

use socket2::{Domain, Protocol, Socket, Type};
use log::{trace, debug, info, error};

/// The port to use for multicast
pub const PORT: u16 = 7645;
lazy_static! {
    /// Multicast IPv4 address
    pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 69).into();

    /// Multicast IPv6 address (technically this is unneeded because Alan disabled IPv6 because of
    /// a rouge DHCP server. Don't know why they don't just go over the logs... Are they hiding
    /// something?)
    pub static ref IPV6: IpAddr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123).into();
}

/// Creates a socket based on the address passed in.
fn create_socket(addr: &SocketAddr) -> Result<Socket, io::Error> {
    let domain = if addr.is_ipv4() {
        Domain::ipv4()
    } else {
        Domain::ipv6()
    };

    let socket = Socket::new(domain, Type::dgram(), Some(Protocol::udp()))?;

    // We're going to use read timeouts so that we don't hang waiting for packets.
    // This is 'merica; we want (need!) it NOW!!!
    // NOTE: This line of code cost me about half an hour, because I forgot that
    // NOTE: this will obviously raise OSE 35.
    // socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    Ok(socket)
}

/// Bind the socket passed in to the address, again, passed in.
fn bind_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&socket2::SockAddr::from(*addr))
}

/// Joins the mulicast <insert terminology>. Create the socket.
fn join_multicast(addr: &SocketAddr) -> Result<UdpSocket, io::Error> {
    let ip_addr = addr.ip();

    let socket = create_socket(&addr)?;

    // Depending on the IP protocol we have slightly different work
    match ip_addr {
        IpAddr::V4(ref mdns_v4) => {
            // Join to the multicast address, with all interfaces
            socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0))?;
        }
        IpAddr::V6(ref mdns_v6) => {
            // Join to the multicast address, with all interfaces (ipv6 uses indexes not addresses)
            socket.join_multicast_v6(mdns_v6, 0)?;
            socket.set_only_v6(true)?;
        }
    };

    // Bind us to the socket address.
    bind_multicast(&socket, &addr)?;

    // Convert to standard sockets
    Ok(socket.into_udp_socket())
}

/// Pretty self explanitory. Spawns a thread that serves to listen
/// to multicast.
pub fn multicast_listener(ipv: IpVersion) -> JoinHandle<()> {
    let addr = SocketAddr::new(match ipv {
        IpVersion::V4 => *IPV4,
        IpVersion::V6 => *IPV6,
    }, PORT);

    debug!("Starting the SLaDOS Multicast Listener thread....");

    let join_handle = std::thread::Builder::new()
        .name("SLaDOS Multicast Listener".to_string())
        .spawn(move || {
            let listener = join_multicast(&addr).expect("failed to create listener");
            info!("Joined multicast at {}!", addr);

            let mut buf = [0u8; 64]; // Receive buffer
            match listener.recv_from(&mut buf) {
                Ok((len, remote_addr)) => {
                    let data = &buf[..len];

                    trace!(
                        "Got data from server: {} from: {}",
                        String::from_utf8_lossy(data),
                        remote_addr
                    );
                },

                Err(err) => {
                    error!("Server got an error. {}", err);
                }
            }
        }).unwrap();

    join_handle
}

/// Test if the IPv4 address is, in fact, multicast capable
#[test]
fn test_ipv4_multicast() {
    assert!(IPV4.is_multicast());
}

/// Test if the IPv6 address is, in fact, multicast capable
#[test]
fn test_ipv6_multicast() {
    assert!(IPV6.is_multicast());
}


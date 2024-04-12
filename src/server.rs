use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub fn create_server() -> TcpListener {
    let host = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8002);

    match TcpListener::bind(host) {
        Ok(server) => server,
        Err(_) => panic!("Couldn't start server at {}:{}", host.ip(), host.port()),
    }
}

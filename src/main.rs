use std::io::{Read, Write};

mod requests;
mod responses;
mod server;

fn main() {
    let tcp_server = server::create_server();
    let mut req_buffer = [0; 512];

    for stream in tcp_server.incoming() {
        let mut stream = stream.unwrap();
        stream.read(&mut req_buffer);
        println!("Request: {}", String::from_utf8_lossy(&req_buffer));
        let response = responses::format_response("THX".to_string(), 200).unwrap();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

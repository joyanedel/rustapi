use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod requests;
mod responses;
mod server;
mod structs;

fn main() {
    let tcp_server = server::create_server();

    for stream in tcp_server.incoming() {
        let mut stream = stream.unwrap();
        handle_request(&mut stream);
        let response = responses::format_response("THX".to_string(), 200).unwrap();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_request(req_stream: &mut TcpStream) {
    let mut req_buffer = [0; 512];
    req_stream.read(&mut req_buffer).unwrap();
    let parsed_stream = String::from_utf8_lossy(&req_buffer);

    let http_lines: Vec<&str> = parsed_stream.split("\n").collect();
    let result = requests::get_http_request_line(&http_lines);

    println!("R: {:?}", result);
}

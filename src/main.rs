use std::{
    io::{Read, Write},
    net::TcpStream,
    thread::{self, sleep},
    time,
};

mod requests;
mod responses;
mod server;
mod structs;

fn main() {
    let tcp_server = server::create_server();

    for stream in tcp_server.incoming() {
        let mut stream = stream.unwrap();
        // handle_request(&mut stream);
        thread::spawn(move || handle_request(&mut stream));
    }
}

fn handle_request(req_stream: &mut TcpStream) {
    let mut req_buffer = [0; 512];
    req_stream.read(&mut req_buffer).unwrap();
    let parsed_stream = String::from_utf8_lossy(&req_buffer);
    sleep(time::Duration::from_secs(1));

    let http_lines: Vec<&str> = parsed_stream.split("\n").collect();
    let http_request = requests::parse_http_request(&http_lines);

    println!("{:#?}", http_request.unwrap());

    let response = responses::format_response("THX".to_string(), 200).unwrap();
    req_stream.write(response.as_bytes()).unwrap();
    req_stream.flush().unwrap();
}

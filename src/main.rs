use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread,
};

mod requests;
mod responses;
mod server;
mod structs;

fn main() {
    let tcp_server = server::create_server();

    for stream in tcp_server.incoming() {
        let stream = stream.unwrap();
        // handle_request(&mut stream);
        thread::spawn(move || handle_request(stream));
    }
}

fn handle_request(mut req_stream: TcpStream) {
    let buf_reader = BufReader::new(req_stream.try_clone().unwrap());
    let http_request_lines: Vec<String> = buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    let http_request = requests::parse_http_request(&http_request_lines);

    println!("{:#?}", http_request.unwrap());

    let response = responses::format_response("THX".to_string(), 200).unwrap();
    req_stream.write(response.as_bytes()).unwrap();
    req_stream.flush().unwrap();
}

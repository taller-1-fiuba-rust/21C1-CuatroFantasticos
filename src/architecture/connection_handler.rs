use std::net::TcpStream;
use std::io::{Read, Write};

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_size = stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
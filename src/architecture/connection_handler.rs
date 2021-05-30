use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _read_size = stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let _write_size = stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

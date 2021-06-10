use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use crate::configuration::verbose::Verbose;

pub fn handle_connection(mut stream: TcpStream, verbose: Verbose) {
    verbose.print("handle_connection");
    let mut buffer = [0; 1024];
    let read_size = stream.read(&mut buffer);
    match read_size {
        Ok(_) => {
            let s = match std::str::from_utf8(&buffer) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("{}", s);
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            let _write_size = stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_e) => {
            verbose.print("HOLIS ESTOY POR DESCONECTARME");
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_size = stream.read(&mut buffer);
    match read_size {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            let _write_size = stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_e) => {
            println!("HOLIS ESTOY POR DESCONECTARME");
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

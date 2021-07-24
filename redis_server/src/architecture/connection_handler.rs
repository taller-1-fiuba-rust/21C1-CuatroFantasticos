use crate::global_resources::GlobalResources;
use crate::request_handler::parser::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
/// handle_connection function, reads data from the TCP stream
/// parses the command and gives the response
/// # Arguments
/// * stream - TCP Stream
/// * conf - Configuration

pub fn handle_connection(mut stream: TcpStream, global_conf: GlobalResources) {
    let verbose = global_conf.get_verbose();
    loop {
        let mut buffer = [0; 1024];

        verbose.print("handle_connection: Waiting for request");
        let read_size = stream.read(&mut buffer);

        match read_size {
            Ok(0) => {
                verbose.print("handle_connection: Read 0 bytes");
                break;
            }
            Ok(_) => {
                let s = match std::str::from_utf8(&buffer) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                verbose.print(&format!("handle_connection: {}", s));

                let storage_accessor = global_conf.get_storage_accessor();
                let parser = Parser::new();
                let command = parser.parse(s.as_ref());
                let message = match command {
                    Ok(s) => match s.execute(storage_accessor) {
                        Ok(v) => v,
                        Err(e) => e,
                    },
                    Err(e) => e,
                };

                stream
                    .write_all(message.as_ref())
                    .expect("Could not write a response");

                stream.flush().unwrap();
            }
            Err(e) => {
                verbose.print("handle_connection: Could not read");
                verbose.print(&format!("{:?}", e));
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

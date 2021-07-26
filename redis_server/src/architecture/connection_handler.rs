use crate::global_resources::GlobalResources;
use crate::request_handler::parser::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
/// handle_connection function, reads data from the TCP stream
/// parses the command and gives the response
/// # Arguments
/// * stream - TCP Stream
/// * conf - Configuration

pub struct ConnectionHandler {
    stream: TcpStream,
    global_resources: GlobalResources,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, global_resources: GlobalResources) -> Self {
        ConnectionHandler {
            stream,
            global_resources,
        }
    }
    pub fn handle_connection(&mut self) {
        let verbose = self.global_resources.get_verbose().expect("There is no verbose");
        loop {
            let mut buffer = [0; 1024];

            verbose.print("handle_connection: Waiting for request");
            let read_size = self.stream.read(&mut buffer);

            match read_size {
                Ok(0) => {
                    verbose.print("handle_connection: Read 0 bytes");
                    break;
                }
                Ok(_) => {
                    let s = match std::str::from_utf8(&buffer) {
                        Ok(v) => v,
                        Err(e) => {
                            verbose.print(&format!(
                                "handle_connection: Invalid UTF-8 sequence: {}",
                                e
                            ));
                            break;
                        }
                    };
                    verbose.print(&format!("handle_connection: {}", s));

                    let parser = Parser::new();
                    let command = parser.parse(s.as_ref());
                    let message = match command {
                        Ok(s) => match s.execute(self.global_resources.clone()) {
                            Ok(v) => v,
                            Err(e) => e,
                        },
                        Err(e) => e,
                    };

                    if self.stream.write_all(message.as_ref()).is_err() {
                        verbose.print("handle_connection: Could not write response");
                        break;
                    }

                    if self.stream.flush().is_err() {
                        verbose.print("handle_connection: Could not flush response");
                        break;
                    }
                }
                Err(e) => {
                    verbose.print("handle_connection: Could not read");
                    verbose.print(&format!("{:?}", e));
                    break;
                }
            }
        }
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}

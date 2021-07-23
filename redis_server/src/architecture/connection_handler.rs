use crate::global_conf::GlobalConf;
use crate::request_handler::parser::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn handle_connection(mut stream: TcpStream, global_conf: GlobalConf) {
    let conf = global_conf
        .get_conf()
        .expect("could not get a configuration");
    loop {
        let mut buffer = [0; 1024];

        conf.verbose("handle_connection: Waiting for request");
        let read_size = stream.read(&mut buffer);

        match read_size {
            Ok(0) => {
                conf.verbose("handle_connection: Read 0 bytes");
                break;
            }
            Ok(_) => {
                let s = match std::str::from_utf8(&buffer) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                conf.verbose(&format!("handle_connection: {}", s));

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
                conf.verbose("handle_connection: Could not read");
                conf.verbose(&format!("{:?}", e));
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

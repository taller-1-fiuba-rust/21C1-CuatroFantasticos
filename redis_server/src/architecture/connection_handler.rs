use crate::configuration::Configuration;
use crate::data::redis_request::RedisRequest;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc;
use crate::request_handler::parser::Parser;
use crate::data::storage_message::StorageMessage;
use crate::data::storage_accessor::StorageAccessor;

pub fn handle_connection(mut stream: TcpStream, mut conf: Configuration) {
    let mut buffer = [0; 1024];

    conf.verbose("handle_connection: Waiting for request");
    let read_size = stream.read(&mut buffer);

    match read_size {
        Ok(_) => {
            let s = match std::str::from_utf8(&buffer) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            conf.verbose(&format!("handle_connection: {}", s));

            let accessor = StorageAccessor::new(conf.get_data_sender().clone());
            let parser = Parser::new();
            let command = parser.parse(s.as_ref()).expect("error al parsear el comando");
            let message = match command.execute(accessor){
                Ok(s) => s,
                Err(e) => e
            };

            stream.write_all(message.as_ref()).expect("Could not write a response");

            stream.flush().unwrap();
        }
        Err(_e) => {
            conf.verbose("handle_connection: There is no request");
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

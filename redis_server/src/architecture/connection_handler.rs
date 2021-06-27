use crate::configuration::Configuration;
use crate::data::redis_request::RedisRequest;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc;
use crate::request_handler::parser::Parser;

pub fn handle_connection(mut stream: TcpStream, mut conf: Configuration) {
    let mut buffer = [0; 1024];

    let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    conf.verbose("handle_connection: Waiting for request");
    let read_size = stream.read(&mut buffer);

    match read_size {
        Ok(_) => {
            let s = match std::str::from_utf8(&buffer) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            conf.verbose(&format!("handle_connection: {}", s));

            let parser = Parser::new();
            let command = parser.parse(s.as_ref()).expect("error al parsear el comando");
            let message = command.execute();
            let request = RedisRequest::new(message.to_owned(), sender);
            match conf.get_data_sender().send(request) {
                Ok(_) => {
                    conf.verbose("Sent request successfully");
                }
                Err(e) => {
                    panic!("Could not send request: {}", e);
                }
            }

            stream.flush().unwrap();
        }
        Err(_e) => {
            conf.verbose("handle_connection: There is no request");
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }

    //hace algo el receiver con lo que recibe
    println!("Recibi esto: {}", receiver.recv().unwrap());
}

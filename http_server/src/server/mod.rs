pub mod handler;
mod router;

use crate::server::router::Router;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

/// Server
///
/// Main implementation of the Http Server
///
/// # Arguments
///
/// * socket_addr - str
///

pub struct Server<'a> {
    socket_addr: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 1024];
            if stream.read(&mut read_buffer).unwrap() == 0 {
                continue;
            }

            let req = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}

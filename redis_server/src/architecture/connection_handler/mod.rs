pub mod client_accessor;

use crate::architecture::connection_handler::client_accessor::ClientAccessor;
use crate::command::RedisCommand;
use crate::global_resources::GlobalResources;
use crate::pub_sub::broadcast::PubSubBroadcastMessage;
use crate::request_handler::parser::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc;

/// handle_connection function, reads data from the TCP stream
/// parses the command and gives the response
/// # Arguments
/// * stream - TCP Stream
/// * conf - Configuration

pub struct ConnectionHandler {
    client_id: usize,
    stream: TcpStream,
    global_resources: GlobalResources,
    state: ClientState,
    pub_sub_receiver: mpsc::Receiver<PubSubBroadcastMessage>,
}

enum ClientState {
    Default,
    PubSub,
}

enum ConnectionState {
    Alive,
    Shutdown,
}

impl ConnectionHandler {
    pub fn new(client_id: usize, stream: TcpStream, global_resources: GlobalResources) -> Self {
        let (tx, rx) = mpsc::channel::<PubSubBroadcastMessage>();
        let mut global_resources = global_resources;
        global_resources.set_client_accessor(ClientAccessor::new(client_id, tx));
        ConnectionHandler {
            client_id,
            stream,
            global_resources,
            state: ClientState::Default,
            pub_sub_receiver: rx,
        }
    }

    pub fn handle_connection(&mut self) {
        loop {
            match self.state {
                ClientState::Default => {
                    if let ConnectionState::Shutdown = self.handle_connection_default() {
                        break;
                    }
                }
                ClientState::PubSub => {
                    if let ConnectionState::Shutdown = self.handle_connection_pubsub() {
                        break;
                    }
                }
            }
        }
        let _ = self.stream.shutdown(Shutdown::Both);
    }

    fn handle_connection_default(&mut self) -> ConnectionState {
        let verbose = self.global_resources.get_verbose();
        verbose.print("handle_connection: Waiting for request");

        let mut buffer = [0; 1024];
        let read_size = self.stream.read(&mut buffer);

        match read_size {
            Ok(0) => {
                verbose.print("handle_connection: Read 0 bytes");
                return ConnectionState::Shutdown;
            }
            Ok(_) => {
                let s = match std::str::from_utf8(&buffer) {
                    Ok(v) => v,
                    Err(e) => {
                        verbose.print(&format!("handle_connection: Invalid UTF-8 sequence: {}", e));
                        return ConnectionState::Shutdown;
                    }
                };
                verbose.print(&format!("handle_connection: {}", s));

                let parser = Parser::new();
                let command = parser.parse(s.as_ref());

                let message = match command {
                    Ok(s) => match s.execute(self.global_resources.clone()) {
                        Ok(v) => {
                            if let RedisCommand::Subscribe(_) = s {
                                self.state = ClientState::PubSub;
                                verbose.print(&format!(
                                    "handle_connection: Client {} entered pubsub state",
                                    self.client_id
                                ));
                            };
                            v
                        }
                        Err(e) => e,
                    },
                    Err(e) => e,
                };

                if self.stream.write_all(message.as_ref()).is_err() {
                    verbose.print("handle_connection: Could not write response");
                    return ConnectionState::Shutdown;
                }

                if self.stream.flush().is_err() {
                    verbose.print("handle_connection: Could not flush response");
                    return ConnectionState::Shutdown;
                }
            }
            Err(e) => {
                verbose.print("handle_connection: Could not read");
                verbose.print(&format!("{:?}", e));
                return ConnectionState::Shutdown;
            }
        }
        ConnectionState::Alive
    }
    fn handle_connection_pubsub(&mut self) -> ConnectionState {
        let verbose = self.global_resources.get_verbose();
        verbose.print(&format!(
            "handle_connection: Client {} waiting in pub sub state",
            self.client_id
        ));
        for pub_sub_message in &self.pub_sub_receiver {
            verbose.print(&format!(
                "handle_connection: Client {} received a pub sub message",
                self.client_id
            ));
            let pub_sub_message = pub_sub_message.protocolize();
            if self.stream.write_all(pub_sub_message.as_ref()).is_err() {
                verbose.print("handle_connection: Could not write response");
                return ConnectionState::Shutdown;
            }

            if self.stream.flush().is_err() {
                verbose.print("handle_connection: Could not flush response");
                return ConnectionState::Shutdown;
            }
        }
        ConnectionState::Alive
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub fn new() -> Result<Client, String> {
        if let Ok(connection) = TcpStream::connect("127.0.0.1:6379") {
            Ok(Client { connection })
        } else {
            Err(String::from("Could not connect to redis server"))
        }
    }

    pub fn execute_command(&mut self, command: String) -> String {
        let _result = self.connection.write_all(command.as_bytes());
        let mut buffer = [0; 1024];
        let read_size = self.connection.read(&mut buffer);
        match read_size {
            Ok(_size) => {
                //tomar resultado de size bytes y convertirlo a un String
                todo!()
            }
            Err(_error) => {
                //capaz panickear, tendria que haber un problema con el server si tira error esto
                todo!()
            }
        }
    }
}

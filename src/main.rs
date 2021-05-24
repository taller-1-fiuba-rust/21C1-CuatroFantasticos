use proyecto_taller_1::architecture::thread_pool::ThreadPool;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const THREAD_MAX_QUANTITY: usize = 1000;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Game over");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_size = stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

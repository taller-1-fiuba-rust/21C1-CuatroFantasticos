use crate::architecture::connection_handler;
use crate::architecture::thread_pool::ThreadPool;
use std::net::TcpListener;

const THREAD_MAX_QUANTITY: usize = 1000;

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            connection_handler::handle_connection(stream);
        });
    }
    println!("Game over");
}

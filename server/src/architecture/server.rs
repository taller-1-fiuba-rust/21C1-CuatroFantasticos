use crate::architecture::connection_handler;
use crate::architecture::thread_pool::ThreadPool;
use std::net::TcpListener;
use crate::configuration::Configuration;

const THREAD_MAX_QUANTITY: usize = 1000;

pub fn run_server(conf: Configuration) {
    let port = conf.get("port").get_port();
    let addr = "127.0.0.1:".to_owned() + port;
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // IMPLEMENTAR EL TIMEOUT
        pool.execute(|| {
            connection_handler::handle_connection(stream);
        });
    }
    println!("Game over");
}

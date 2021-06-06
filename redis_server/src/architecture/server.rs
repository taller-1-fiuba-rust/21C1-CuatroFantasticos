use crate::architecture::connection_handler;
use crate::architecture::thread_pool::ThreadPool;
use crate::configuration::Configuration;
use std::net::TcpListener;
use std::time::Duration;

const THREAD_MAX_QUANTITY: usize = 1000;

pub fn run_server(conf: Configuration) {
    let addr = "127.0.0.1:".to_owned() + conf.get("port");
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let timeout = conf.get("timeout").parse().unwrap();
        if timeout != 0 {
            stream
                .set_read_timeout(Some(Duration::new(timeout, 0)))
                .unwrap();
        }

        pool.execute(|| {
            connection_handler::handle_connection(stream);
        });
    }
    println!("Game over");
}

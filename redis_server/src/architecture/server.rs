use crate::architecture::connection_handler;
use crate::architecture::thread_pool::ThreadPool;
use crate::configuration::verbose::Verbose;
use crate::configuration::Configuration;
use std::net::TcpListener;
use std::time::Duration;

const THREAD_MAX_QUANTITY: usize = 1000;

pub fn run_server(conf: Configuration) {
    let status = conf.get("verbose").expect("No se registró verbose en conf");
    let verbose = Verbose::new(status);
    verbose.print("run_server");

    let port = conf.get("port").expect("No se registró un port en conf");
    let addr = "127.0.0.1:".to_owned() + port;

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let timeoutout = conf.get("timeout");
        match timeoutout {
            Some(..) => println!("TIMEOUT ES LO QUE ES"),
            None => println!("TIMEOUT ES CERO"),
        }

        let timeout = timeoutout.expect("asd").parse().unwrap();
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

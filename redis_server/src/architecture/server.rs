use crate::architecture::connection_handler;
use crate::architecture::thread_pool::ThreadPool;
use crate::configuration::verbose::Verbose;
use crate::configuration::Configuration;
use std::net::TcpListener;
use std::time::Duration;

const THREAD_MAX_QUANTITY: usize = 1000;

pub fn run_server(conf: Configuration) {
    let status = conf.get("verbose").unwrap();
    let verbose = Verbose::new(status);
    verbose.print("run_server");

    let port = conf.get("port").expect("No se registró un port en conf.");
    let addr = "127.0.0.1:".to_owned() + port;
    verbose.print("run_server: se conectó al puerto");

    let listener = TcpListener::bind(addr).expect("No se pudo realizar la conexión.");
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY);
    verbose.print("run_server: se realizó la conexión con éxito");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let timeout = conf.get("timeout").unwrap().parse().unwrap();
        let _result = stream.set_read_timeout(Some(Duration::new(timeout, 0)));

        pool.execute(move || {
            connection_handler::handle_connection(stream, verbose);
        });
    }
    println!("Game over");
}

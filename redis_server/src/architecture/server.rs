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
    verbose.print(&format!(
        "run_server: Starting server with configuration \n {:?}",
        conf
    ));

    let port = conf.get("port").expect("There is no port in Configuration");
    let addr = "127.0.0.1:".to_owned() + port;
    verbose.print(&format!("run_server: connecting to {}", addr));

    let listener = TcpListener::bind(addr).expect("Server was not able to connect");
    let pool = ThreadPool::new(THREAD_MAX_QUANTITY, verbose);
    verbose.print("run_server: Succesfully connected");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let timeout = conf.get("timeout").unwrap().parse().unwrap();
        let _result = stream.set_read_timeout(Some(Duration::new(timeout, 0)));

        pool.execute(move || {
            connection_handler::handle_connection(stream, verbose);
        });
    }
    verbose.print("Game over");
}

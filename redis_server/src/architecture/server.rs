use crate::architecture::connection_handler;
use crate::global_conf::GlobalConf;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

pub fn run_server(global_conf: GlobalConf) {
    let conf = global_conf
        .get_conf()
        .expect("could not get a configuration");
    conf.verbose(&format!(
        "run_server: Starting server with configuration \n {:?}",
        conf
    ));
    let port = conf.get("port").expect("There is no port in Configuration");
    let addr = "127.0.0.1:".to_owned() + &port;

    conf.verbose(&format!("run_server: connecting to {}", addr));

    let listener = TcpListener::bind(addr).expect("Server was not able to connect");
    conf.verbose("run_server: Succesfully connected");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let timeout = conf.get("timeout").unwrap().parse().unwrap();
        let _result = stream.set_read_timeout(Some(Duration::new(timeout, 0)));
        let global_conf_thread = global_conf.clone();

        thread::spawn(move || {
            connection_handler::handle_connection(stream, global_conf_thread);
        });
    }
    conf.verbose("Game over");
}

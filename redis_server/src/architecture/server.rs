use crate::architecture::connection_handler::ConnectionHandler;
use crate::global_resources::GlobalResources;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Run Server Sets the port the server will use,
/// Starts a server for webhook
/// # Arguments
/// * conf - Configuration

pub fn run_server(global_resources: GlobalResources) {
    let conf = global_resources
        .get_conf()
        .expect("run_server: Could not get a configuration");
    let verbose = global_resources.get_verbose();
    verbose.print(&format!(
        "run_server: Starting server with configuration \n {:?}",
        conf
    ));
    let port = conf
        .get("port")
        .expect("run_server: There is no port in Configuration");
    let addr = "127.0.0.1:".to_owned() + port;

    verbose.print(&format!("run_server: connecting to {}", addr));

    let listener = TcpListener::bind(addr).expect("run_server :Server was not able to connect");
    verbose.print("run_server: Succesfully connected");

    for (client_id, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();

        let timeout = conf.get("timeout").unwrap().parse().unwrap();
        let _result = stream.set_read_timeout(Some(Duration::new(timeout, 0)));
        let global_resources = global_resources.clone();

        thread::spawn(move || {
            let mut connection_handler =
                ConnectionHandler::new(client_id, stream, global_resources);
            connection_handler.handle_connection();
        });
    }
    verbose.print("run_server: Game over");
}

use http_server::server::Server;

/// main function, creates and runs the server

fn main() {
    let server = Server::new("localhost:8080");
    server.run();
}

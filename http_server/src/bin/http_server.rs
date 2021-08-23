use http_server::server::Server;

fn main() {
    let server = Server::new("localhost:8080");
    server.run();
}

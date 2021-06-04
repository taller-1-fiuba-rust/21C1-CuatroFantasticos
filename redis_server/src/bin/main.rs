extern crate redis_server;

use crate::redis_server::architecture::server;

fn main() {
    server::run_server();
}

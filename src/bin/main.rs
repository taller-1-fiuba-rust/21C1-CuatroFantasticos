use redis_server::architecture::server;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];


    server::run_server();

}

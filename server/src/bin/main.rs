use server::architecture::server;
use server::configuration::Configuration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let conf = Configuration::new(filename);

    server::run_server(conf);
}

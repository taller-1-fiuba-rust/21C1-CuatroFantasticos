extern crate logger;
extern crate redis_server;

use logger::log::LogService;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use std::env;
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut conf: Configuration = Configuration::new(filename);
    let logfile = conf.get("logfile").expect("No hay un logfile definido");
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(logfile)
        .expect("No se pudo crear un archivo de logs");
    let log_service: LogService = LogService::new(file);
    conf.set_logservice(log_service.create_logger());

    server::run_server(&conf);
}

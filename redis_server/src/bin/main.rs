extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;

use logger::log::LogService;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use redis_server::data::storage_service::StorageService;

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

    let dbfilename = conf.get("dbfilename").unwrap();
    let db_file = OpenOptions::new()
        .read(true)
        .open(dbfilename)
        .expect("No se pudo crear un archivo de logs");

    let storage_service = StorageService::new(db_file);
    conf.set_data_sender(storage_service.get_storage_sender());
    server::run_server(&conf);
}

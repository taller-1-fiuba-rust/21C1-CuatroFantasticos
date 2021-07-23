extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;

use logger::log_service::LogService;
use redis_server::architecture::server;
use redis_server::data::storage_service::StorageService;
use redis_server::configuration::conf_service::ConfService;
use redis_server::configuration::conf_accesor::ConfAccessor;
use redis_server::configuration::conf_request_message::ConfRequestMessage;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let conf_service = ConfService::new(filename.clone());

    let (conf_sender , conf_receiver) = mpsc::Channel::<ConfRequestMessage>();
    let conf_accesor = ConfAccessor::new();



    let logfile = conf.get("logfile").expect("No hay un logfile definido");
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(logfile)
        .expect("No se pudo crear un archivo de logs");
    let log_service = LogService::new(file);
    conf.set_logger_builder(log_service.get_log_interface());

    let dbfilename = conf.get("dbfilename").unwrap();
    let db_file = OpenOptions::new()
        .read(true)
        .open(dbfilename)
        .expect("No se pudo crear un archivo de database");

    let storage_service = StorageService::new(db_file);
    conf.set_data_sender(storage_service.get_storage_sender());
    server::run_server(&conf);
}

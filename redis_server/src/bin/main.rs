extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;
use std::sync::mpsc;
use std::thread;

use logger::log::LogService;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use redis_server::data::storage_service::operator_service::request_message::StorageRequestMessage;
use redis_server::data::storage_service::operator_service::StorageOperatorService;

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

    let (sender, receiver): (
        mpsc::Sender<StorageRequestMessage>,
        mpsc::Receiver<StorageRequestMessage>,
    ) = mpsc::channel();

    conf.set_data_sender(sender);
    let dbfilename = conf.get("dbfilename").unwrap();
    thread::spawn(move || {
        let storage = StorageOperatorService::new(&dbfilename, receiver);
        //storage.print();
        storage.init();
    });
    server::run_server(&conf);
}

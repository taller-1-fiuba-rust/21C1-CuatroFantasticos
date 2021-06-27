extern crate logger;
extern crate redis_server;

use logger::log::LogService;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use redis_server::data::redis_request::RedisRequest;
use redis_server::data::storage::Storage;
use redis_server::request_handler::parser::Parser;
use std::env;
use std::fs::OpenOptions;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

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

    let (sender, receiver): (mpsc::Sender<RedisRequest>, mpsc::Receiver<RedisRequest>) =
        mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    conf.set_data_sender(sender);
    let storage = Storage::new(conf.get("dbfilename").unwrap());
    storage.imprimir();
    thread::spawn(move || loop {
        let receiver = Arc::clone(&receiver);
        let mut request = receiver.lock().unwrap().recv().unwrap();
        request
            .get_sender()
            .send("ESTO FUNCIONA? siiiiii".to_string())
            .unwrap();
    });

    server::run_server(&conf);

    let _parser = Parser::new();
}

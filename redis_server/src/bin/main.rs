extern crate logger;
extern crate redis_server;

use logger::log::LogService;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use std::env;
use std::fs::OpenOptions;
use std::sync::mpsc;
use std::thread;
use redis_server::data::data_receiver::DataReceiver;
use redis_server::data::redis_request::RedisRequest;

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

    let (sender, receiver): (mpsc::Sender<RedisRequest>, mpsc::Receiver<RedisRequest>) = mpsc::channel();

    conf.set_data_sender(sender);

    thread::spawn(|| {
        let mut receiver = DataReceiver::new(receiver);
        let mut request = receiver.receive();
        request.get_sender().send("ESTO FUNCIONA?".to_string()).unwrap();
    });




    server::run_server(&conf);
}

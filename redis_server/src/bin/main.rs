extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;

use logger::log_service::LogService;
use redis_server::architecture::server;
use redis_server::configuration::service::ConfService;
use redis_server::data::storage_service::StorageService;
use redis_server::global_conf::GlobalConf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let conf_service = ConfService::new(filename.clone());
    let conf = conf_service.get_conf().unwrap();
    let logfilename = conf.get("logfile").expect("couldn't get a logfile");
    let dbfilename = conf.get("dbfilename").expect("couldn't get a dbfilename");
    let log_service = LogService::new_with_path(&logfilename);

    let db_file = OpenOptions::new()
        .read(true)
        .open(dbfilename)
        .expect("No se pudo crear un archivo de database");

    let storage_service = StorageService::new(db_file);
    let global_conf = GlobalConf::new(
        log_service.get_log_interface(),
        conf_service.get_accessor_builder(),
        storage_service.get_accessor_builder(),
    );
    server::run_server(global_conf);
}

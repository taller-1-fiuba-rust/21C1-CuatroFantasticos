extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;

use logger::log_service::LogService;
use redis_server::architecture::server;
use redis_server::configuration::service::ConfService;
use redis_server::configuration::verbose::Verbose;
use redis_server::data::storage::service::StorageService;
use redis_server::global_resources::GlobalResources;
use redis_server::pub_sub::service::PubSubService;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let conf_service = ConfService::new(filename);
    let conf = conf_service.get_conf().unwrap();
    let log_filename = conf.get("logfile").expect("main: Couldn't get a logfile");
    let verbose_conf = conf.get("verbose").expect("main: Couldn't get a logfile");
    let db_filename = conf
        .get("dbfilename")
        .expect("main: Couldn't get a dbfilename");
    let log_service = LogService::new_with_path(log_filename);
    let verbose = Verbose::new(verbose_conf);

    let db_file = OpenOptions::new()
        .read(true)
        .open(db_filename)
        .expect("main: Couldn't open database file");

    let storage_service = StorageService::new(db_file);
    let pub_sub_service = PubSubService::new();
    let global_conf = GlobalResources::new(
        log_service.get_log_interface(),
        verbose,
        conf_service.get_accessor_builder(),
        storage_service.get_accessor_builder(),
        pub_sub_service.get_accessor_builder(),
    );
    server::run_server(global_conf);
}

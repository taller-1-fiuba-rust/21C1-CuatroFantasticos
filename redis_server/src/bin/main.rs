extern crate logger;
extern crate redis_server;

use std::env;
use std::fs::OpenOptions;

use logger::log_service::LogService;
use redis_server::architecture::server;
use redis_server::configuration::service::ConfService;
use redis_server::data::storage::service::StorageService;
use redis_server::global_resources::GlobalResources;
use redis_server::pub_sub::service::PubSubService;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut global_resources = GlobalResources::new();

    let conf_service = ConfService::new(filename);
    let conf_accessor = conf_service.get_accessor_builder();
    global_resources.add_conf_access_builder(conf_accessor);

    let conf = conf_service.get_conf().unwrap();
    let log_filename = conf.get("logfile").expect("main: Couldn't get a logfile");
    let db_filename = conf
        .get("dbfilename")
        .expect("main: Couldn't get a dbfilename");
    let log_service = LogService::new_with_path(log_filename);
    let logger_builder = log_service.get_log_interface();
    global_resources.add_logger_builder(logger_builder);

    let db_file = OpenOptions::new()
        .read(true)
        .open(db_filename)
        .expect("main: Couldn't open database file");
    let storage_service = StorageService::new(db_file);
    let storage_accessor_builder = storage_service.get_accessor_builder();
    global_resources.add_storage_access_builder(storage_accessor_builder);

    let pub_sub_service = PubSubService::new();
    let pub_sub_access_builder = pub_sub_service.get_accessor_builder();
    global_resources.add_pub_sub_access_builder(pub_sub_access_builder);
    server::run_server(global_resources);
}

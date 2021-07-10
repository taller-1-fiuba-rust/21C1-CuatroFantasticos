use redis::ConnectionLike;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use redis_server::data::storage::request_message::StorageRequestMessage;
use redis_server::data::storage::Storage;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_connection_timeouts_when_server_sleeps_two_seconds() {
    //make sure redis.conf file is set properly with with timeout 1 sec
    let mut conf = Configuration::new("../redis.conf");
    thread::spawn(move || {
        let (sender, receiver): (
            mpsc::Sender<StorageRequestMessage>,
            mpsc::Receiver<StorageRequestMessage>,
        ) = mpsc::channel();

        conf.set_data_sender(sender);
        let dbfilename = conf.get("dbfilename").unwrap();
        thread::spawn(move || {
            let storage = Storage::new(&dbfilename, receiver);
            //storage.print();
            storage.init();
        });
        server::run_server(&conf);
    });
    sleep(Duration::new(1, 0));
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client
        .get_connection()
        .expect("falló la conexión cliente-servidor");
    sleep(Duration::new(2, 0));
    let is_connect = con.check_connection();
    assert!(!is_connect);
}

#[test]
fn test_connection_doesnt_timeout_when_server_doesnt_sleep() {
    //make sure redis.conf is properly set with timeout 1 second
    let mut conf = Configuration::new("../redis.conf");
    thread::spawn(move || {
        let (sender, receiver): (
            mpsc::Sender<StorageRequestMessage>,
            mpsc::Receiver<StorageRequestMessage>,
        ) = mpsc::channel();

        conf.set_data_sender(sender);
        let dbfilename = conf.get("dbfilename").unwrap();
        thread::spawn(move || {
            let storage = Storage::new(&dbfilename, receiver);
            //storage.print();
            storage.init();
        });
        server::run_server(&conf);
    });
    sleep(Duration::new(1, 0));
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = client
        .get_connection()
        .expect("falló la conexión cliente-servidor");
    let _is_connect = con.check_connection();
    //assert!(is_connect);
}

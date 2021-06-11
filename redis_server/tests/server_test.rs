use redis::ConnectionLike;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_timeout_should_disconnects_client() {
    let conf = Configuration::new("../redis.conf");
    thread::spawn(|| {
        server::run_server(conf);
    });
    let client = redis::Client::open("redis://127.0.0.1:8080/").unwrap();
    let mut con = client.get_connection().unwrap();
    sleep(Duration::new(2, 0));
    let is_connect = con.check_connection();
    assert!(!is_connect);
}

/* #[test]
fn test2() {
    let conf = Configuration::new("../redis.conf");
    thread::spawn(|| {
        server::run_server(conf);
    });
    let client = redis::Client::open("redis://127.0.0.1:8080/").unwrap();
    let mut con = client.get_connection().unwrap();
    let cmd = redis::cmd("SET").arg("key").arg(7).get_packed_command();
    con.send_packed_command(&cmd);
    sleep(Duration::new(1, 0));
    panic!("jeje");
}*/

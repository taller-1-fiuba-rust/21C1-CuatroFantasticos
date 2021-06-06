use redis::ConnectionLike;
use redis_server::architecture::server;
use redis_server::configuration::Configuration;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test() {
    let conf = Configuration::new("../redis.conf");
    thread::spawn(|| {
        server::run_server(conf);
    });
    let client = redis::Client::open("redis://127.0.0.1:7878/").unwrap();
    let mut con = client.get_connection().unwrap();
    sleep(Duration::new(4, 0));
    let is_connect = con.check_connection();
    assert!(!is_connect);
}

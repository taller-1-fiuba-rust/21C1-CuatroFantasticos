use std::sync::mpsc;
use crate::data::redis_request::RedisRequest;
use std::sync::{Arc, Mutex};

pub struct DataReceiver {
    receiver: Arc<Mutex<mpsc::Receiver<RedisRequest>>>,
}

impl DataReceiver {
    pub fn new(receiver: mpsc::Receiver<RedisRequest>) -> Self {
        let receiver = Arc::new(Mutex::new(receiver));
        DataReceiver{
            receiver
        }
    }

    pub fn receive(&mut self) -> RedisRequest{
        self.receiver.lock().unwrap().recv().unwrap()
    }

}
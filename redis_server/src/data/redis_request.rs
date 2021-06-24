use std::sync::mpsc;

#[derive(Clone)]
pub struct RedisRequest {
    message: String,
    sender: mpsc::Sender<String>,
}

impl RedisRequest{
    pub fn new(message: String, sender : mpsc::Sender<String>) -> RedisRequest{
        RedisRequest{
            message,
            sender,
        }
    }

    pub fn get_sender(&mut self) -> &mpsc::Sender<String> {
        &self.sender
    }
}
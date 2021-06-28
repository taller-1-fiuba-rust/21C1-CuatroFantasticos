use std::sync::mpsc;

pub struct StorageMessage {
    message: StorageMessageEnum,
    sender: mpsc::Sender<String>,
}

impl StorageMessage {
    pub fn new(message: StorageMessageEnum, sender: mpsc::Sender<String>) -> StorageMessage {
        StorageMessage { message, sender }
    }

    pub fn getMessage(&self) -> StorageMessageEnum {
        self.message.clone()
    }

    pub fn getSender(&self) -> mpsc::Sender<String> {
        self.sender.clone()
    }
}

#[derive(Clone)]
pub enum StorageMessageEnum {
    GetDbsize,
    Terminate,
}

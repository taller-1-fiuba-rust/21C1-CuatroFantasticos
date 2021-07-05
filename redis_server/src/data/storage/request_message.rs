use crate::data::storage::response_message::StorageResponseMessage;
use std::sync::mpsc;

pub struct StorageRequestMessage {
    message: StorageRequestMessageEnum,
    sender: mpsc::Sender<StorageResponseMessage>,
}

impl StorageRequestMessage {
    pub fn new(
        message: StorageRequestMessageEnum,
        sender: mpsc::Sender<StorageResponseMessage>,
    ) -> StorageRequestMessage {
        StorageRequestMessage { message, sender }
    }

    pub fn get_message(&self) -> StorageRequestMessageEnum {
        self.message.clone()
    }

    pub fn get_sender(&self) -> mpsc::Sender<StorageResponseMessage> {
        self.sender.clone()
    }
}

#[derive(Clone)]
pub enum StorageRequestMessageEnum {
    GetDbsize,
    FlushDb,
    Exists(String),
    Rename(String, String),
    Terminate,
}

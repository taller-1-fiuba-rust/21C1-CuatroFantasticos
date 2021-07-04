use crate::data::storage_response::StorageResponse;
use std::sync::mpsc;

pub struct StorageMessage {
    message: StorageMessageEnum,
    sender: mpsc::Sender<StorageResponse>,
}

impl StorageMessage {
    pub fn new(
        message: StorageMessageEnum,
        sender: mpsc::Sender<StorageResponse>,
    ) -> StorageMessage {
        StorageMessage { message, sender }
    }

    pub fn get_message(&self) -> StorageMessageEnum {
        self.message.clone()
    }

    pub fn get_sender(&self) -> mpsc::Sender<StorageResponse> {
        self.sender.clone()
    }
}

#[derive(Clone)]
pub enum StorageMessageEnum {
    GetDbsize,
    FlushDb,
    Exists(String),
    Rename(String, String),
    Terminate,
}

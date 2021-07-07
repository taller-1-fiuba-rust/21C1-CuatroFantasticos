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

    fn get_sender(&self) -> mpsc::Sender<StorageResponseMessage> {
        self.sender.clone()
    }

    pub fn respond(&self, response: StorageResponseMessage) -> Result<(), String> {
        self.get_sender()
            .send(response)
            .map_err(|_| "Client thread is not listening to storage response".to_string())
    }
}

#[derive(Clone)]
pub enum StorageRequestMessageEnum {
    GetDbsize,
    FlushDb,
    Get(String),
    Copy(String, String),
    Exists(String),
    Rename(String, String),
    Type(String),
    Del(String),
    Terminate,
    Append(String, String),
    GetDel(String),
    GetSet(String, String),
    Strlen(String),
}

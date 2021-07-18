use crate::data::storage_service::operator_service::response_message::{
    StorageResponseMessage, StorageResult,
};
use std::sync::mpsc;

pub struct StorageRequestMessage {
    message: StorageAction,
    sender: Option<mpsc::Sender<StorageResponseMessage>>,
}

impl StorageRequestMessage {
    pub fn new(
        message: StorageAction,
        sender: Option<mpsc::Sender<StorageResponseMessage>>,
    ) -> StorageRequestMessage {
        StorageRequestMessage { message, sender }
    }

    pub fn get_message(&self) -> StorageAction {
        self.message.clone()
    }

    pub fn respond(&self, response: StorageResult) -> Result<(), String> {
        match &self.sender {
            Some(sender) => {
                let response_message = StorageResponseMessage::new(response);
                sender
                    .send(response_message)
                    .map_err(|_| "Client thread is not listening to storage response".to_string())
            }
            None => Err("There is no sender present to respond".to_string()),
        }
    }
}

#[derive(Clone)]
pub enum StorageAction {
    Dbsize,
    FlushDb,
    Get(String),
    Lindex(String, i32),
    Copy(String, String),
    Exists(String),
    Rename(String, String),
    Type(String),
    Del(String),
    Append(String, String),
    GetDel(String),
    GetSet(String, String),
    Strlen(String),
    Llen(String),
    ExpirationRound,
    Persist,
    Terminate,
    DecrBy(String, i32),
    IncrBy(String, i32),
    Touch(String),
}

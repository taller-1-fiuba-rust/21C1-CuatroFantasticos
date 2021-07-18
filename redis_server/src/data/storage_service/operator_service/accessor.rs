use crate::data::storage_service::operator_service::request_message::{
    StorageAction, StorageRequestMessage,
};
use crate::data::storage_service::operator_service::response_message::StorageResponseMessage;
use std::sync::mpsc;

pub struct StorageAccessor {
    sender: mpsc::Sender<StorageRequestMessage>,
    sender_for_storage: mpsc::Sender<StorageResponseMessage>,
    receiver: mpsc::Receiver<StorageResponseMessage>,
}

impl StorageAccessor {
    pub fn new(sender: mpsc::Sender<StorageRequestMessage>) -> StorageAccessor {
        let (sender_for_storage, receiver) = mpsc::channel::<StorageResponseMessage>();

        StorageAccessor {
            sender,
            sender_for_storage,
            receiver,
        }
    }

    pub fn access(&self, message: StorageAction) -> Result<StorageResponseMessage, String> {
        let storage_message =
            StorageRequestMessage::new(message, Some(self.sender_for_storage.clone()));
        match self.sender.send(storage_message) {
            Ok(_) => Ok(self.receiver.recv().unwrap()),
            Err(_) => Err("Error sending message to storage".to_string()),
        }
    }
}

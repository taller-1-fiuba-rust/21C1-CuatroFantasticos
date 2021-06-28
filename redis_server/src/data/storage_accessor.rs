use crate::data::storage_message::{StorageMessage, StorageMessageEnum};
use std::sync::mpsc;

pub struct StorageAccessor {
    sender: mpsc::Sender<StorageMessage>,
    sender_for_storage: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

impl StorageAccessor {
    pub fn new(sender: mpsc::Sender<StorageMessage>) -> StorageAccessor {
        let (sender_for_storage, receiver) = mpsc::channel::<String>();

        StorageAccessor {
            sender,
            sender_for_storage,
            receiver,
        }
    }

    pub fn access(&self, message: StorageMessageEnum) -> Result<String, String> {
        let storage_message = StorageMessage::new(message, self.sender_for_storage.clone());
        self.sender
            .send(storage_message)
            .map_err(|_| "Error sending message to storage".to_string());
        Ok(self.receiver.recv().unwrap())
    }
}

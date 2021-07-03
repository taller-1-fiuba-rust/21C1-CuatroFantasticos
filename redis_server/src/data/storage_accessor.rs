use crate::data::storage_message::{StorageMessage, StorageMessageEnum};
use crate::data::storage_response::StorageResponse;
use std::sync::mpsc;

pub struct StorageAccessor {
    sender: mpsc::Sender<StorageMessage>,
    sender_for_storage: mpsc::Sender<StorageResponse>,
    receiver: mpsc::Receiver<StorageResponse>,
}

impl StorageAccessor {
    pub fn new(sender: mpsc::Sender<StorageMessage>) -> StorageAccessor {
        let (sender_for_storage, receiver) = mpsc::channel::<StorageResponse>();

        StorageAccessor {
            sender,
            sender_for_storage,
            receiver,
        }
    }

    pub fn access(&self, message: StorageMessageEnum) -> Result<StorageResponse, String> {
        let storage_message = StorageMessage::new(message, self.sender_for_storage.clone());
        match self
            .sender
            .send(storage_message)
            .map_err(|_| "Error sending message to storage".to_string())
        {
            Ok(_) => Ok(self.receiver.recv().unwrap()),
            Err(e) => Err(e),
        }
    }
}

use crate::data::storage::request_message::{StorageRequestMessage, StorageRequestMessageEnum};
use crate::data::storage::response_message::StorageResponseMessage;
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

    pub fn access(
        &self,
        message: StorageRequestMessageEnum,
    ) -> Result<StorageResponseMessage, String> {
        let storage_message = StorageRequestMessage::new(message, self.sender_for_storage.clone());
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

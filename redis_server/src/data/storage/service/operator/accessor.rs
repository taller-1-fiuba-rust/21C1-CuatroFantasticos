use crate::data::storage::service::operator::request_message::{
    StorageAction, StorageRequestMessage,
};
use crate::data::storage::service::operator::response_message::StorageResponseMessage;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;

pub struct StorageAccessor {
    sender: mpsc::Sender<StorageRequestMessage>,
    sender_for_storage: mpsc::Sender<StorageResponseMessage>,
    receiver: mpsc::Receiver<StorageResponseMessage>,
}
pub enum StorageAccessorError {
    SendError,
    ReceiveError,
}

impl Debug for StorageAccessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageAccessorError::SendError => {
                write!(f, "Error accessing storage")
            }
            StorageAccessorError::ReceiveError => {
                write!(f, "Error getting result from storage")
            }
        }
    }
}

impl From<StorageAccessorError> for String {
    fn from(e: StorageAccessorError) -> Self {
        format!("{:?}", e)
    }
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
        message: StorageAction,
    ) -> Result<StorageResponseMessage, StorageAccessorError> {
        let storage_message =
            StorageRequestMessage::new(message, Some(self.sender_for_storage.clone()));
        match self.sender.send(storage_message) {
            Ok(_) => self
                .receiver
                .recv()
                .map_err(|_| StorageAccessorError::ReceiveError),
            Err(_) => Err(StorageAccessorError::SendError),
        }
    }
}

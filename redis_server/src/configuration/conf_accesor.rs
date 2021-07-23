use std::sync::mpsc;
use std::fmt::{Debug, Formatter};
use crate::configuration::conf_request_message::{ConfMessage, ConfRequestMessage};
use crate::configuration::conf_response_message::ConfResult;

pub struct ConfAccessor {
    sender: mpsc::Sender<ConfRequestMessage>,
    sender_for_worker: mpsc::Sender<ConfResult>,
    receiver: mpsc::Receiver<ConfResult>,
}
pub enum ConfAccessorError {
    SendError,
    ReceiveError,
}

impl Debug for ConfAccessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfAccessorError::SendError => {
                write!(f, "Error accessing conf")
            }
            ConfAccessorError::ReceiveError => {
                write!(f, "Error getting result from conf")
            }
        }
    }
}

impl From<ConfAccessorError> for String {
    fn from(e: ConfAccessorError) -> Self {
        format!("{:?}", e)
    }
}

impl ConfAccessor {
    pub fn new(sender: mpsc::Sender<ConfRequestMessage>) -> ConfAccessor {
        let (sender_for_worker, receiver) = mpsc::channel::<ConfResult>();

        ConfAccessor {
            sender,
            sender_for_worker,
            receiver,
        }
    }

    pub fn access(
        &self,
        message: ConfMessage,
    ) -> Result<ConfResult, ConfAccessorError> {
        let storage_message =
            ConfRequestMessage::new(message, Some(self.sender_for_worker.clone()));
        match self.sender.send(storage_message) {
            Ok(_) => self
                .receiver
                .recv()
                .map_err(|_| ConfAccessorError::ReceiveError),
            Err(_) => Err(ConfAccessorError::SendError),
        }
    }
}

use crate::pub_sub::service::action::PubSubAction;
use crate::pub_sub::service::request_message::PubSubOperatorRequest;
use crate::pub_sub::service::result::PubSubResult;
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;

pub struct PubSubAccessor {
    sender: mpsc::Sender<PubSubOperatorRequest>,
    sender_for_pub_sub: mpsc::Sender<PubSubResult>,
    receiver: mpsc::Receiver<PubSubResult>,
}
pub enum PubSubAccessorError {
    SendError,
    ReceiveError,
}

impl Debug for PubSubAccessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PubSubAccessorError::SendError => {
                write!(f, "Error accessing pub sub")
            }
            PubSubAccessorError::ReceiveError => {
                write!(f, "Error getting result from pub sub")
            }
        }
    }
}

impl From<PubSubAccessorError> for String {
    fn from(e: PubSubAccessorError) -> Self {
        format!("{:?}", e)
    }
}

impl PubSubAccessor {
    pub fn new(sender: mpsc::Sender<PubSubOperatorRequest>) -> PubSubAccessor {
        let (sender_for_pub_sub, receiver) = mpsc::channel::<PubSubResult>();

        PubSubAccessor {
            sender,
            sender_for_pub_sub,
            receiver,
        }
    }

    pub fn access(&self, message: PubSubAction) -> Result<PubSubResult, PubSubAccessorError> {
        let message = PubSubOperatorRequest::new(message, Some(self.sender_for_pub_sub.clone()));
        match self.sender.send(message) {
            Ok(_) => self
                .receiver
                .recv()
                .map_err(|_| PubSubAccessorError::ReceiveError),
            Err(_) => Err(PubSubAccessorError::SendError),
        }
    }
}

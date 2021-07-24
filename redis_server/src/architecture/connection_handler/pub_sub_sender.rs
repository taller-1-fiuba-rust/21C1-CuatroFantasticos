use crate::pub_sub::broadcast::PubSubBroadcast;
use std::sync::mpsc;

#[derive(Clone)]
pub struct ClientPubSubSender {
    sender: mpsc::Sender<PubSubBroadcast>,
}

pub enum ClientPubSubSenderError {
    SendError,
}

impl ClientPubSubSender {
    pub fn new(sender: mpsc::Sender<PubSubBroadcast>) -> Self {
        ClientPubSubSender { sender }
    }
    pub fn send(&self, message: PubSubBroadcast) -> Result<(), ClientPubSubSenderError> {
        self.sender
            .send(message)
            .map_err(|_| ClientPubSubSenderError::SendError)
    }
}

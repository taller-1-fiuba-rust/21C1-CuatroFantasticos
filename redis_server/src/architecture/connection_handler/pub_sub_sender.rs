use crate::pub_sub::broadcast::PubSubBroadcastMessage;
use std::sync::mpsc;

#[derive(Clone)]
pub struct ClientPubSubSender {
    sender: mpsc::Sender<PubSubBroadcastMessage>,
}

pub enum ClientPubSubSenderError {
    SendError,
}

impl ClientPubSubSender {
    pub fn new(sender: mpsc::Sender<PubSubBroadcastMessage>) -> Self {
        ClientPubSubSender { sender }
    }
    pub fn send(&self, message: PubSubBroadcastMessage) -> Result<(), ClientPubSubSenderError> {
        self.sender
            .send(message)
            .map_err(|_| ClientPubSubSenderError::SendError)
    }
    pub fn protocolize(&self) {}
}

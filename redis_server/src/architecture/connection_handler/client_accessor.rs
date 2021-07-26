use crate::pub_sub::broadcast::PubSubBroadcastMessage;
use std::sync::mpsc;

#[derive(Clone, Debug)]
pub struct ClientAccessor {
    id: usize,
    sender: mpsc::Sender<PubSubBroadcastMessage>,
}

pub enum ClientPubSubSenderError {
    SendError,
}

impl ClientAccessor {
    pub fn new(id: usize, sender: mpsc::Sender<PubSubBroadcastMessage>) -> Self {
        ClientAccessor { id, sender }
    }
    pub fn send(&self, message: PubSubBroadcastMessage) -> Result<(), ClientPubSubSenderError> {
        let result = self.sender.send(message);
        println!("Error{:?}", result);
        result.map_err(|_| ClientPubSubSenderError::SendError)
    }

    pub fn get_client_id(&self) -> usize {
        self.id
    }
}

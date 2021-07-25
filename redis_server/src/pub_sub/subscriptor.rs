use crate::architecture::connection_handler::pub_sub_sender::ClientPubSubSender;
use crate::pub_sub::service::broadcast::PubSubBroadcastMessage;
use std::hash::{Hash, Hasher};

pub struct PubSubSubscriptor {
    client_id: usize,
    client_sender: ClientPubSubSender,
}

pub enum PubSubSubscriptorError {
    SendError,
}

impl PubSubSubscriptor {
    pub fn new(client_id: usize, client_sender: ClientPubSubSender) -> Self {
        PubSubSubscriptor {
            client_id,
            client_sender,
        }
    }
    pub fn send(&self, message: PubSubBroadcastMessage) -> Result<(), PubSubSubscriptorError> {
        self.client_sender
            .send(message)
            .map_err(|_| PubSubSubscriptorError::SendError)
    }
}

impl PartialEq for PubSubSubscriptor {
    fn eq(&self, other: &Self) -> bool {
        self.client_id == other.client_id
    }
}

impl Eq for PubSubSubscriptor {}

impl Hash for PubSubSubscriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.client_id.hash(state)
    }
}

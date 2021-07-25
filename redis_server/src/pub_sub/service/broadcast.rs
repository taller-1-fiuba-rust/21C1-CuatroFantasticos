#[derive(Clone)]
pub struct PubSubBroadcastMessage {
    message: String,
}

impl PubSubBroadcastMessage {
    pub fn new(message: String) -> Self {
        PubSubBroadcastMessage { message }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

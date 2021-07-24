pub struct PubSubBroadcast {
    message: String,
}

impl PubSubBroadcast {
    pub fn new(message: String) -> Self {
        PubSubBroadcast { message }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

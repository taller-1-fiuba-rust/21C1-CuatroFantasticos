use crate::protocol_serialization::ProtocolSerializer;

#[derive(Clone)]
pub struct PubSubBroadcastMessage {
    channel: String,
    message: String,
}

impl PubSubBroadcastMessage {
    pub fn new(channel: &str, message: &str) -> Self {
        PubSubBroadcastMessage {
            channel: channel.to_owned(),
            message: message.to_owned(),
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn protocolize(&self) -> String {
        vec!["message", &self.channel, &self.message].protocol_serialize_to_bulk_string()
    }
}

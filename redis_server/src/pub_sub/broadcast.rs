use crate::protocol_serialization::ProtocolSerializer;

#[derive(Clone)]
pub struct PubSubBroadcastMessage {
    message: String,
    channel: String,
}

impl PubSubBroadcastMessage {
    pub fn new(message: &str, channel: &str) -> Self {
        PubSubBroadcastMessage {
            message: message.to_owned(),
            channel: channel.to_owned(),
        }
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn protocolize(&self) -> String {
        vec!["message", &self.channel, &self.message].protocol_serialize_to_bulk_string()
    }
}

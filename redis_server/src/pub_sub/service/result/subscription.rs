use crate::protocol_serialization::ProtocolSerializer;

pub struct PubSubSubscriptionResult {
    channel: String,
    qty_subscriptions: usize,
}

impl PubSubSubscriptionResult {
    pub fn new(channel: &str, qty_subscriptions: usize) -> Self {
        PubSubSubscriptionResult {
            channel: channel.to_owned(),
            qty_subscriptions,
        }
    }
    pub fn protocolize(&self) -> String {
        let vec_str = "*3\r\n".to_owned();
        let subscribe_str = "subscribe".protocol_serialize_to_bulk_string();
        let channel_str = self.channel.protocol_serialize_to_bulk_string();
        let qty_str = self
            .qty_subscriptions
            .to_string()
            .protocol_serialize_to_int();
        vec_str + &subscribe_str + &channel_str + &qty_str
    }
}

#[cfg(test)]
mod tests {
    use crate::pub_sub::service::result::subscription::PubSubSubscriptionResult;

    #[test]
    fn pub_sub_subscription_result_protocolization_works() {
        let result = PubSubSubscriptionResult::new("juanete", 5).protocolize();
        let protocolized_string = "*3\r\n$9\r\nsubscribe\r\n$7\r\njuanete\r\n:5\r\n";
        assert_eq!(result, protocolized_string);
    }
}

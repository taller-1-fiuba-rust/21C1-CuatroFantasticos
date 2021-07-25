use crate::pub_sub::channel::PubSubChannel;
use crate::pub_sub::service::broadcast::PubSubBroadcastMessage;
use crate::pub_sub::subscriptor::PubSubSubscriptor;
use std::collections::HashMap;

#[derive(Default)]
pub struct PubSubOrchestrator {
    channels: HashMap<String, PubSubChannel>,
}

impl PubSubOrchestrator {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn subscribe_to_channel(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        if !self.channels.contains_key(channel) {
            self.channels
                .insert(channel.to_owned(), PubSubChannel::new());
        }
        if let Some(channel) = self.channels.get_mut(channel) {
            channel.add_subscriptor(subscriptor)
        }
    }
    pub fn unsubscribe_from_channel(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        if !self.channels.contains_key(channel) {
            return;
        }
        if let Some(channel) = self.channels.get_mut(channel) {
            channel.remove_subscriptor(&subscriptor)
        }
    }
    pub fn publish_to_channel(&mut self, message: &str, channel_name: &str) {
        if !self.channels.contains_key(channel_name) {
            return;
        }
        if let Some(channel) = self.channels.get_mut(channel_name) {
            let message = PubSubBroadcastMessage::new(message, channel_name);
            channel.broadcast(message);
        }
    }
}

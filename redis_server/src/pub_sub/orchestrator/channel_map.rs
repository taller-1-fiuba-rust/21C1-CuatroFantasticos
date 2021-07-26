use crate::pub_sub::broadcast::PubSubBroadcastMessage;
use crate::pub_sub::channel::PubSubChannel;
use crate::pub_sub::subscriptor::PubSubSubscriptor;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ChannelMap {
    channels: HashMap<String, PubSubChannel>,
}

impl ChannelMap {
    pub fn subscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        match self.channels.get_mut(channel) {
            None => {
                let mut channel_struct = PubSubChannel::new();
                channel_struct.add_subscriptor(subscriptor);
                self.channels.insert(channel.to_owned(), channel_struct);
            }
            Some(channel_struct) => {
                channel_struct.add_subscriptor(subscriptor);
            }
        }
    }
    pub fn unsubscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        match self.channels.get_mut(channel) {
            None => {}
            Some(channel) => channel.remove_subscriptor(&subscriptor),
        }
    }
    pub fn publish(&mut self, channel: &str, message: &str) -> usize {
        match self.channels.get_mut(channel) {
            None => 0,
            Some(channel_struct) => {
                let message = PubSubBroadcastMessage::new(channel, message);
                channel_struct.broadcast(message)
            }
        }
    }
}

mod channel_map;
mod subscriptor_map;

use crate::pub_sub::orchestrator::channel_map::ChannelMap;
use crate::pub_sub::orchestrator::subscriptor_map::SubscriptorMap;
use crate::pub_sub::subscriptor::PubSubSubscriptor;

#[derive(Default, Debug)]
pub struct PubSubOrchestrator {
    channels: ChannelMap,
    subscriptors: SubscriptorMap,
}

impl PubSubOrchestrator {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn subscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) -> usize {
        self.channels.subscribe(subscriptor.clone(), channel);
        self.subscriptors.subscribe(subscriptor.clone(), channel);
        self.subscriptors.subscription_qty(&subscriptor)
    }
    pub fn unsubscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) -> usize {
        self.channels.unsubscribe(subscriptor.clone(), channel);
        self.subscriptors.unsubscribe(subscriptor.clone(), channel);
        self.subscriptors.subscription_qty(&subscriptor)
    }
    pub fn publish(&mut self, channel: &str, message: &str) -> usize {
        self.channels.publish(channel, message)
    }
}

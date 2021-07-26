use crate::pub_sub::subscriptor::PubSubSubscriptor;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct SubscriptorMap {
    subscriptors: HashMap<PubSubSubscriptor, HashSet<String>>,
}

impl SubscriptorMap {
    pub fn subscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        match self.subscriptors.get_mut(&subscriptor) {
            None => {
                let mut set = HashSet::new();
                set.insert(channel.to_owned());
                self.subscriptors.insert(subscriptor, set);
            }
            Some(channels) => {
                channels.insert(channel.to_owned());
            }
        }
    }
    pub fn unsubscribe(&mut self, subscriptor: PubSubSubscriptor, channel: &str) {
        match self.subscriptors.get_mut(&subscriptor) {
            None => {}
            Some(channels) => {
                channels.remove(channel);
            }
        }
    }
    pub fn subscription_qty(&self, subscriptor: &PubSubSubscriptor) -> usize {
        match self.subscriptors.get(subscriptor) {
            None => 0,
            Some(channels) => channels.len(),
        }
    }
}

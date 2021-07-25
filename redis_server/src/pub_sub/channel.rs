use crate::pub_sub::service::broadcast::PubSubBroadcastMessage;
use crate::pub_sub::subscriptor::PubSubSubscriptor;
use std::collections::HashSet;

#[derive(Default)]
pub struct PubSubChannel {
    broadcasting_list: HashSet<PubSubSubscriptor>,
}

impl PubSubChannel {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_subscriptor(&mut self, subscriptor: PubSubSubscriptor) {
        self.broadcasting_list.replace(subscriptor);
    }
    pub fn remove_subscriptor(&mut self, subscriptor: &PubSubSubscriptor) {
        self.broadcasting_list.remove(subscriptor);
    }
    pub fn broadcast(&mut self, message: PubSubBroadcastMessage) {
        for subscriptor in &self.broadcasting_list {
            let _ = subscriptor.send(message.clone());
        }
    }
}

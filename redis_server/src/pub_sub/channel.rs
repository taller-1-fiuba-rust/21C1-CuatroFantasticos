use crate::pub_sub::broadcast::PubSubBroadcastMessage;
use crate::pub_sub::subscriptor::PubSubSubscriptor;
use std::collections::HashSet;

#[derive(Default, Debug)]
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
    pub fn broadcast(&mut self, message: PubSubBroadcastMessage) -> usize {
        let mut receiver_qty: usize = 0;
        for subscriptor in &self.broadcasting_list {
            if subscriptor.send(message.clone()).is_ok() {
                receiver_qty += 1;
            }
        }
        receiver_qty
    }
}

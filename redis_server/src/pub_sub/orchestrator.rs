use crate::pub_sub::channel::PubSubChannel;
use std::collections::HashMap;

#[derive(Default)]
pub struct PubSubOrchestrator {
    _channels: HashMap<String, PubSubChannel>,
}

impl PubSubOrchestrator {
    pub fn new() -> Self {
        Default::default()
    }
}

use crate::pub_sub::service::accessor::PubSubAccessor;
use crate::pub_sub::service::request_message::PubSubOperatorRequest;
use std::sync::mpsc;

#[derive(Clone)]
pub struct PubSubAccessorBuilder {
    sender: mpsc::Sender<PubSubOperatorRequest>,
}

impl PubSubAccessorBuilder {
    pub fn new(sender: mpsc::Sender<PubSubOperatorRequest>) -> Self {
        PubSubAccessorBuilder { sender }
    }
    pub fn build_accessor(&self) -> PubSubAccessor {
        PubSubAccessor::new(self.sender.clone())
    }
}

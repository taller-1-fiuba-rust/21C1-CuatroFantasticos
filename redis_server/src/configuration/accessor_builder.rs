use crate::configuration::accesor::ConfAccessor;
use crate::configuration::request_message::ConfRequestMessage;
use std::sync::mpsc;

#[derive(Clone)]
pub struct ConfAccessorBuilder {
    sender: mpsc::Sender<ConfRequestMessage>,
}

impl ConfAccessorBuilder {
    pub fn new(sender: mpsc::Sender<ConfRequestMessage>) -> Self {
        ConfAccessorBuilder { sender }
    }
    pub fn build_accessor(&self) -> ConfAccessor {
        ConfAccessor::new(self.sender.clone())
    }
}

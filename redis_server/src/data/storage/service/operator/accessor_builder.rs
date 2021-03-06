use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageRequestMessage;
use std::sync::mpsc;

#[derive(Clone)]
pub struct StorageAccessorBuilder {
    sender: mpsc::Sender<StorageRequestMessage>,
}

impl StorageAccessorBuilder {
    pub fn new(sender: mpsc::Sender<StorageRequestMessage>) -> Self {
        StorageAccessorBuilder { sender }
    }
    pub fn build_accessor(&self) -> StorageAccessor {
        StorageAccessor::new(self.sender.clone())
    }
}

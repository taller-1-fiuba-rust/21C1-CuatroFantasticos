use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use std::sync::mpsc;

pub struct StorageExpirationService {
    accessor: StorageAccessor,
    receiver: mpsc::Receiver<ExpirationServiceMessage>,
    period: u128,
}

pub enum ExpirationServiceMessage {
    Terminate,
}

impl StorageExpirationService {
    pub fn new(
        accessor: StorageAccessor,
        receiver: mpsc::Receiver<ExpirationServiceMessage>,
        period: u128,
    ) -> StorageExpirationService {
        StorageExpirationService {
            accessor,
            receiver,
            period,
        }
    }

    pub fn init(&self) {
        loop {
            let message = self.receiver.try_recv();
            if let Ok(ExpirationServiceMessage::Terminate) = message {
                break;
            }
        }
    }
}

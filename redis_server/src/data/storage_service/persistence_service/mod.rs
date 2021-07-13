use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use std::sync::mpsc;

pub struct StoragePersistenceService {
    accessor: StorageAccessor,
    receiver: mpsc::Receiver<PersistenceServiceMessage>,
    period: u128,
}

pub enum PersistenceServiceMessage {
    Terminate,
}

impl StoragePersistenceService {
    pub fn new(
        accessor: StorageAccessor,
        receiver: mpsc::Receiver<PersistenceServiceMessage>,
        period: u128,
    ) -> StoragePersistenceService {
        StoragePersistenceService {
            accessor,
            receiver,
            period,
        }
    }

    pub fn init(&self) {
        loop {
            let message = self.receiver.try_recv();
            if let Ok(PersistenceServiceMessage::Terminate) = message {
                break;
            }
        }
    }
}

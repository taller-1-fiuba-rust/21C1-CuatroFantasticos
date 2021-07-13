use crate::data::storage_service::operator_service::accessor::StorageAccessor;

pub struct StoragePersistenceService {
    accessor: StorageAccessor,
}

impl StoragePersistenceService {
    pub fn new(accessor: StorageAccessor) -> StoragePersistenceService {
        StoragePersistenceService { accessor }
    }

    pub fn init(&self) {
        todo!()
    }
}

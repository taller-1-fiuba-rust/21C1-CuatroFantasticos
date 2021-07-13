use crate::data::storage_service::operator_service::accessor::StorageAccessor;

pub struct StorageExpirationService {
    accessor: StorageAccessor,
}

impl StorageExpirationService {
    pub fn new(accessor: StorageAccessor) -> StorageExpirationService {
        StorageExpirationService { accessor }
    }

    pub fn init(&self) {
        todo!()
    }
}

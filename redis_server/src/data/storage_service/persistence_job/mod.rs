use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::job_recurser_service::{RecurringJob, RecurringJobError};

pub struct PersistenceJob {
    accessor: StorageAccessor,
}

impl PersistenceJob {
    pub fn new(accessor: StorageAccessor) -> Self {
        PersistenceJob { accessor }
    }
}

impl RecurringJob for PersistenceJob {
    fn execute_job(&self) -> Result<(), RecurringJobError> {
        match self.accessor.access(StorageAction::Persist) {
            Ok(_) => Ok(()),
            Err(_) => Err(RecurringJobError {}),
        }
    }
}

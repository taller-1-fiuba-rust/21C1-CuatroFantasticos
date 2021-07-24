use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
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
    fn execute_job(&mut self) -> Result<(), RecurringJobError> {
        match self.accessor.access(StorageAction::Save) {
            Ok(_) => Ok(()),
            Err(_) => Err(RecurringJobError {}),
        }
    }
}

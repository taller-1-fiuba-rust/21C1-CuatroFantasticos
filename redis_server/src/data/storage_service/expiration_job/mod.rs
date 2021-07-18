use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::job_recurser_service::{RecurringJob, RecurringJobError};

pub struct ExpirationJob {
    accessor: StorageAccessor,
}

impl ExpirationJob {
    pub fn new(accessor: StorageAccessor) -> Self {
        ExpirationJob { accessor }
    }
}

impl RecurringJob for ExpirationJob {
    fn execute_job(&self) -> Result<(), RecurringJobError> {
        match self.accessor.access(StorageAction::ExpirationRound) {
            Ok(_) => Ok(()),
            Err(_) => Err(RecurringJobError {}),
        }
    }
}

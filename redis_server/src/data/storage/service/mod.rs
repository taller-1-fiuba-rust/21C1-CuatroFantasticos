use std::io::Read;
use std::sync::mpsc;
use std::thread;

use crate::data::storage::service::expiration_job::ExpirationJob;

use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::accessor_builder::StorageAccessorBuilder;
use crate::data::storage::service::operator::request_message::{
    StorageAction, StorageRequestMessage,
};
use crate::data::storage::service::operator::StorageOperatorService;
use crate::data::storage::service::persistence_job::PersistenceJob;
use crate::global_resources::GlobalResources;
use crate::job_recurser_service::JobRecurserService;

mod expiration_job;
pub mod operator;
mod persistence_job;

const EXPIRATION_PERIOD_IN_MILLIS: u128 = 2 * 1000;
const PERSISTENCE_PERIOD_IN_MILLIS: u128 = 10 * 1000;

pub struct StorageService {
    operator_request_sender: mpsc::Sender<StorageRequestMessage>,
    operator_thread_handler: Option<thread::JoinHandle<()>>,
    _global_resources: GlobalResources,
    _expiration_service: JobRecurserService,
    _persistence_service: JobRecurserService,
}

impl StorageService {
    pub fn new<T>(persistence_object: T, global_resources: GlobalResources) -> Self
    where
        T: Read + Send + 'static,
    {
        let (operator_tx, operator_rx) = mpsc::channel::<StorageRequestMessage>();
        let global_resources_copied = global_resources.clone();
        let operator_th = thread::spawn(move || {
            let storage = StorageOperatorService::new(
                persistence_object,
                operator_rx,
                global_resources_copied,
            );
            storage.init();
        });

        let expiration_accessor = StorageAccessor::new(operator_tx.clone());
        let expiration_service = JobRecurserService::new(
            ExpirationJob::new(expiration_accessor),
            EXPIRATION_PERIOD_IN_MILLIS,
        );

        let persistence_accessor = StorageAccessor::new(operator_tx.clone());
        let persistence_service = JobRecurserService::new(
            PersistenceJob::new(persistence_accessor),
            PERSISTENCE_PERIOD_IN_MILLIS,
        );

        StorageService {
            operator_request_sender: operator_tx,
            operator_thread_handler: Some(operator_th),
            _global_resources: global_resources,
            _expiration_service: expiration_service,
            _persistence_service: persistence_service,
        }
    }

    pub fn get_accessor_builder(&self) -> StorageAccessorBuilder {
        StorageAccessorBuilder::new(self.operator_request_sender.clone())
    }
}

impl Drop for StorageService {
    fn drop(&mut self) {
        let _ = self
            .operator_request_sender
            .send(StorageRequestMessage::new(StorageAction::Terminate, None));
        if let Some(th) = self.operator_thread_handler.take() {
            th.join().unwrap();
        }
    }
}

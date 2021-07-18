use crate::data::storage_service::expiration_job::ExpirationJob;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::{
    StorageAction, StorageRequestMessage,
};
use crate::data::storage_service::operator_service::StorageOperatorService;
use crate::data::storage_service::persistence_job::PersistenceJob;
use crate::job_recurser_service::JobRecurserService;
use std::io::Read;
use std::sync::mpsc;
use std::thread;

mod expiration_job;
pub mod operator_service;
mod persistence_job;

pub struct StorageService {
    operator_request_sender: mpsc::Sender<StorageRequestMessage>,
    operator_thread_handler: Option<thread::JoinHandle<()>>,
    _expiration_service: JobRecurserService,
    _persistence_service: JobRecurserService,
}

impl StorageService {
    pub fn new<T>(persistence_object: T) -> Self
    where
        T: Read + Send + 'static,
    {
        let (operator_tx, operator_rx) = mpsc::channel::<StorageRequestMessage>();

        let operator_th = thread::spawn(move || {
            let storage = StorageOperatorService::new(persistence_object, operator_rx);
            storage.init();
        });

        let expiration_accessor = StorageAccessor::new(operator_tx.clone());
        let _expiration_service =
            JobRecurserService::new(ExpirationJob::new(expiration_accessor), 250);

        let persistence_accessor = StorageAccessor::new(operator_tx.clone());
        let _persistence_service =
            JobRecurserService::new(PersistenceJob::new(persistence_accessor), 300 * 1000);

        StorageService {
            operator_request_sender: operator_tx,
            operator_thread_handler: Some(operator_th),
            _expiration_service,
            _persistence_service,
        }
    }

    pub fn get_storage_sender(&self) -> mpsc::Sender<StorageRequestMessage> {
        self.operator_request_sender.clone()
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

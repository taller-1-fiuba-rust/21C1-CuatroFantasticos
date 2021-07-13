use crate::data::storage_service::expiration_service::{
    ExpirationServiceMessage, StorageExpirationService,
};
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::{
    StorageRequestMessage, StorageRequestMessageEnum,
};
use crate::data::storage_service::operator_service::StorageOperatorService;
use crate::data::storage_service::persistence_service::{
    PersistenceServiceMessage, StoragePersistenceService,
};
use std::io::Read;
use std::sync::mpsc;
use std::thread;

mod expiration_service;
pub mod operator_service;
mod persistence_service;

pub struct StorageService {
    operator_request_sender: mpsc::Sender<StorageRequestMessage>,
    operator_thread_handler: Option<thread::JoinHandle<()>>,
    expiration_request_sender: mpsc::Sender<ExpirationServiceMessage>,
    expiration_thread_handler: Option<thread::JoinHandle<()>>,
    persistence_request_sender: mpsc::Sender<PersistenceServiceMessage>,
    persistence_thread_handler: Option<thread::JoinHandle<()>>,
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

        let (expiration_tx, expiration_rx) = mpsc::channel::<ExpirationServiceMessage>();
        let expiration_accessor = StorageAccessor::new(operator_tx.clone());
        let expiration_th = thread::spawn(move || {
            let expiration = StorageExpirationService::new(expiration_accessor, expiration_rx, 250);
            expiration.init();
        });

        let (persistence_tx, persistence_rx) = mpsc::channel::<PersistenceServiceMessage>();
        let persistence_accessor = StorageAccessor::new(operator_tx.clone());
        let persistence_th = thread::spawn(move || {
            let persistence =
                StoragePersistenceService::new(persistence_accessor, persistence_rx, 300 * 1000);
            persistence.init();
        });

        StorageService {
            operator_request_sender: operator_tx,
            operator_thread_handler: Some(operator_th),
            expiration_request_sender: expiration_tx,
            expiration_thread_handler: Some(expiration_th),
            persistence_request_sender: persistence_tx,
            persistence_thread_handler: Some(persistence_th),
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
            .send(StorageRequestMessage::new(
                StorageRequestMessageEnum::Terminate,
                None,
            ));
        if let Some(th) = self.operator_thread_handler.take() {
            th.join().unwrap();
        }

        let _ = self
            .expiration_request_sender
            .send(ExpirationServiceMessage::Terminate);
        if let Some(th) = self.expiration_thread_handler.take() {
            th.join().unwrap();
        }

        let _ = self
            .persistence_request_sender
            .send(PersistenceServiceMessage::Terminate);
        if let Some(th) = self.persistence_thread_handler.take() {
            th.join().unwrap();
        }
    }
}

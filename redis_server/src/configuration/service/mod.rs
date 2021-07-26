use std::sync::mpsc;
use std::thread;

use accesor::ConfAccessor;
use accessor_builder::ConfAccessorBuilder;
use response_message::ConfResult;
use worker::ConfWorker;

use crate::configuration::service::request_message::{ConfAction, ConfRequestMessage};
use crate::configuration::service::service_error::ConfServiceError;
use crate::configuration::Configuration;

pub mod accesor;
pub mod accessor_builder;
pub mod request_message;
pub mod response_message;
mod service_error;
mod worker;

pub struct ConfService {
    conf_request_sender: mpsc::Sender<ConfRequestMessage>,
    conf_thread_handler: Option<thread::JoinHandle<()>>,
}

impl ConfService {
    pub fn new(conf_filename: &str) -> Self {
        let configuration = Configuration::new(conf_filename);
        let (conf_tx, conf_rx) = mpsc::channel::<ConfRequestMessage>();

        let conf_th = thread::spawn(move || {
            let conf = ConfWorker::new(conf_rx, configuration);
            conf.init();
        });

        ConfService {
            conf_request_sender: conf_tx,
            conf_thread_handler: Some(conf_th),
        }
    }

    pub fn get_accessor_builder(&self) -> ConfAccessorBuilder {
        ConfAccessorBuilder::new(self.conf_request_sender.clone())
    }

    pub fn get_conf(&self) -> Result<Configuration, ConfServiceError> {
        let accessor = ConfAccessor::new(self.conf_request_sender.clone());
        match accessor.access(ConfAction::Get) {
            Ok(ConfResult::OkConf(value)) => Ok(value),
            _ => Err(ConfServiceError::GetConfError),
        }
    }
}

impl Drop for ConfService {
    fn drop(&mut self) {
        let _ = self
            .conf_request_sender
            .send(ConfRequestMessage::new(ConfAction::Terminate, None));
        if let Some(th) = self.conf_thread_handler.take() {
            th.join().unwrap();
        }
    }
}

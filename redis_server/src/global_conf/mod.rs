use std::sync::mpsc;

use crate::configuration::conf_accesor::ConfAccessor;
use crate::configuration::conf_request_message::{ConfMessage, ConfRequestMessage};
use crate::configuration::conf_response_message::ConfResult;
use crate::configuration::Configuration;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessage;
use logger::log_service::log_interface::LogInterface;
use logger::log_service::logger::Logger;
use std::fs::File;

#[derive(Clone)]
pub struct GlobalConf {
    logger_builder: LogInterface<File>,
    configuration_sender: mpsc::Sender<ConfRequestMessage>,
    storage_sender: mpsc::Sender<StorageRequestMessage>,
}

impl GlobalConf {
    pub fn new(
        logger_builder: LogInterface<File>,
        configuration_sender: mpsc::Sender<ConfRequestMessage>,
        storage_sender: mpsc::Sender<StorageRequestMessage>,
    ) -> Self {
        GlobalConf {
            logger_builder,
            configuration_sender,
            storage_sender,
        }
    }

    pub fn get_storage_accessor(&self) -> StorageAccessor {
        StorageAccessor::new(self.storage_sender.clone())
    }

    pub fn get_configuration_accessor(&self) -> ConfAccessor {
        ConfAccessor::new(self.configuration_sender.clone())
    }

    pub fn get_logger(&self) -> Logger<File> {
        self.logger_builder.build_logger()
    }

    pub fn get_conf(&self) -> Result<Configuration, String> {
        let accessor = ConfAccessor::new(self.configuration_sender.clone());
        match accessor.access(ConfMessage::Get).unwrap() {
            ConfResult::OkConf(value) => Ok(value),
            _ => Err(String::from("Couldn't get a configuration")),
        }
    }
}

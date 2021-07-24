use crate::configuration::accessor_builder::ConfAccessorBuilder;
use crate::configuration::conf_accesor::ConfAccessor;
use crate::configuration::conf_request_message::ConfMessage;
use crate::configuration::conf_response_message::ConfResult;
use crate::configuration::Configuration;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::accessor_builder::StorageAccessorBuilder;
use logger::log_service::log_interface::LogInterface;
use logger::log_service::logger::Logger;
use std::fs::File;

#[derive(Clone)]
pub struct GlobalConf {
    logger_builder: LogInterface<File>,
    configuration_access_builder: ConfAccessorBuilder,
    storage_access_builder: StorageAccessorBuilder,
}

impl GlobalConf {
    pub fn new(
        logger_builder: LogInterface<File>,
        configuration_sender: ConfAccessorBuilder,
        storage_sender: StorageAccessorBuilder,
    ) -> Self {
        GlobalConf {
            logger_builder,
            configuration_access_builder: configuration_sender,
            storage_access_builder: storage_sender,
        }
    }

    pub fn get_storage_accessor(&self) -> StorageAccessor {
        self.storage_access_builder.build_accessor()
    }

    pub fn get_configuration_accessor(&self) -> ConfAccessor {
        self.configuration_access_builder.build_accessor()
    }

    pub fn get_logger(&self) -> Logger<File> {
        self.logger_builder.build_logger()
    }

    pub fn get_conf(&self) -> Result<Configuration, String> {
        let accessor = self.configuration_access_builder.build_accessor();
        match accessor.access(ConfMessage::Get).unwrap() {
            ConfResult::OkConf(value) => Ok(value),
            _ => Err(String::from("Couldn't get a configuration")),
        }
    }
}

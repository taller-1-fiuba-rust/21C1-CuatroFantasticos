mod error;

use crate::configuration::service::accesor::ConfAccessor;
use crate::configuration::service::accessor_builder::ConfAccessorBuilder;
use crate::configuration::service::request_message::ConfAction;
use crate::configuration::service::response_message::ConfResult;
use crate::configuration::verbose::Verbose;
use crate::configuration::Configuration;

use crate::architecture::connection_handler::pub_sub_sender::ClientPubSubSender;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::accessor_builder::StorageAccessorBuilder;
use crate::global_resources::error::GlobalResourcesError;
use logger::log_service::log_interface::LogInterface;
use logger::log_service::logger::Logger;
use std::fs::File;

#[derive(Clone)]
pub struct GlobalResources {
    logger_builder: LogInterface<File>,
    verbose: Verbose,
    configuration_access_builder: ConfAccessorBuilder,
    storage_access_builder: StorageAccessorBuilder,
    client_pub_sub_sender: Option<ClientPubSubSender>,
}

impl GlobalResources {
    pub fn new(
        logger_builder: LogInterface<File>,
        verbose: Verbose,
        configuration_sender: ConfAccessorBuilder,
        storage_sender: StorageAccessorBuilder,
    ) -> Self {
        GlobalResources {
            logger_builder,
            verbose,
            configuration_access_builder: configuration_sender,
            storage_access_builder: storage_sender,
            client_pub_sub_sender: None,
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

    pub fn get_verbose(&self) -> Verbose {
        self.verbose
    }

    pub fn get_conf(&self) -> Result<Configuration, GlobalResourcesError> {
        let accessor = self.configuration_access_builder.build_accessor();
        match accessor.access(ConfAction::Get) {
            Ok(ConfResult::OkConf(value)) => Ok(value),
            _ => Err(GlobalResourcesError::GetConfError),
        }
    }
    pub fn set_client_pub_sub_sender(&mut self, client_pub_sub_sender: ClientPubSubSender) {
        self.client_pub_sub_sender = Some(client_pub_sub_sender);
    }
    pub fn get_client_pub_sub_sender(&self) -> Option<ClientPubSubSender> {
        self.client_pub_sub_sender.as_ref().cloned()
    }
}

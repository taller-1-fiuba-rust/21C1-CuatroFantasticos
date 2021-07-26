mod error;

use crate::configuration::service::accesor::ConfAccessor;
use crate::configuration::service::accessor_builder::ConfAccessorBuilder;
use crate::configuration::service::request_message::ConfAction;
use crate::configuration::service::response_message::ConfResult;
use crate::configuration::verbose::Verbose;
use crate::configuration::Configuration;

use crate::architecture::connection_handler::client_accessor::ClientAccessor;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::accessor_builder::StorageAccessorBuilder;
use crate::global_resources::error::GlobalResourcesError;
use crate::pub_sub::service::accessor::PubSubAccessor;
use crate::pub_sub::service::accessor_builder::PubSubAccessorBuilder;
use logger::log_service::log_interface::LogInterface;
use logger::log_service::logger::Logger;
use std::fs::File;

#[derive(Clone)]
pub struct GlobalResources {
    logger_builder: LogInterface<File>,
    verbose: Verbose,
    configuration_access_builder: ConfAccessorBuilder,
    storage_access_builder: StorageAccessorBuilder,
    pub_sub_access_builder: PubSubAccessorBuilder,
    client_pub_sub_sender: Option<ClientAccessor>,
}

impl GlobalResources {
    pub fn new(
        logger_builder: LogInterface<File>,
        verbose: Verbose,
        configuration_access_builder: ConfAccessorBuilder,
        storage_access_builder: StorageAccessorBuilder,
        pub_sub_access_builder: PubSubAccessorBuilder,
    ) -> Self {
        GlobalResources {
            logger_builder,
            verbose,
            configuration_access_builder,
            storage_access_builder,
            pub_sub_access_builder,
            client_pub_sub_sender: None,
        }
    }

    pub fn get_storage_accessor(&self) -> StorageAccessor {
        self.storage_access_builder.build_accessor()
    }

    pub fn get_pub_sub_accessor(&self) -> PubSubAccessor {
        self.pub_sub_access_builder.build_accessor()
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
    pub fn set_client_accessor(&mut self, client_pub_sub_sender: ClientAccessor) {
        self.client_pub_sub_sender = Some(client_pub_sub_sender);
    }
    pub fn get_client_accessor(&self) -> Option<ClientAccessor> {
        self.client_pub_sub_sender.as_ref().cloned()
    }
}

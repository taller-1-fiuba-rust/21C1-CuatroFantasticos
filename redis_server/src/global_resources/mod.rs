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
    logger_builder: Option<LogInterface<File>>,
    configuration_access_builder: Option<ConfAccessorBuilder>,
    storage_access_builder: Option<StorageAccessorBuilder>,
    pub_sub_access_builder: Option<PubSubAccessorBuilder>,
    client_pub_sub_sender: Option<ClientAccessor>,
}

impl GlobalResources {
    pub fn new() -> GlobalResources {
        GlobalResources {
            logger_builder: None,
            configuration_access_builder: None,
            storage_access_builder: None,
            pub_sub_access_builder: None,
            client_pub_sub_sender: None,
        }
    }

    pub fn add_logger_builder(&mut self, logger_builder: LogInterface<File>) {
        self.logger_builder = Some(logger_builder);
    }

    pub fn add_conf_access_builder(&mut self, conf_access_builder: ConfAccessorBuilder) {
        self.configuration_access_builder = Some(conf_access_builder);
    }

    pub fn add_storage_access_builder(&mut self, storage_access_builder: StorageAccessorBuilder) {
        self.storage_access_builder = Some(storage_access_builder);
    }

    pub fn add_pub_sub_access_builder(&mut self, pub_sub_access_builder: PubSubAccessorBuilder) {
        self.pub_sub_access_builder = Some(pub_sub_access_builder);
    }

    pub fn get_storage_accessor(&self) -> StorageAccessor {
        let builder = self
            .storage_access_builder
            .as_ref()
            .expect("There is no storage accessor builder");
        builder.build_accessor()
    }

    pub fn get_pub_sub_accessor(&self) -> PubSubAccessor {
        self.pub_sub_access_builder
            .as_ref()
            .expect("No pub sub access builder")
            .build_accessor()
    }

    pub fn get_configuration_accessor(&self) -> ConfAccessor {
        let builder = self
            .configuration_access_builder
            .as_ref()
            .expect("There is no configuration accessor builder");
        builder.build_accessor()
    }

    pub fn get_logger(&self) -> Logger<File> {
        let builder = self
            .logger_builder
            .as_ref()
            .expect("There is no logger builder");
        builder.build_logger()
    }

    pub fn get_verbose(&self) -> Result<Verbose, String> {
        let accessor = self
            .configuration_access_builder
            .as_ref()
            .expect("there is no conf accessor")
            .build_accessor();
        match accessor.access(ConfAction::GetVerbose)? {
            ConfResult::Verbose(value) => Ok(value),
            _ => Err("There is no verbose".to_string()),
        }
    }

    pub fn get_dbfilename(&self) -> Result<String,String> {
        let accessor = self
            .configuration_access_builder
            .as_ref()
            .expect("there is no conf accessor")
            .build_accessor();
        match accessor.access(ConfAction::GetParameters("dbfilename".to_string()))? {
            ConfResult::Vector(value) => Ok(value.get(1).expect("could not get dbfilename").to_owned()),
            _ => Err("There is no verbose".to_string()),
        }

    }

    pub fn get_conf(&self) -> Result<Configuration, GlobalResourcesError> {
        let accessor = self
            .configuration_access_builder
            .as_ref()
            .expect("There is no conf accessor")
            .build_accessor();
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

impl Default for GlobalResources {
    fn default() -> Self {
        Self::new()
    }
}

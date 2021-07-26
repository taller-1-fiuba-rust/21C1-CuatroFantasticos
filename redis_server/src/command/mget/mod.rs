use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandMGet {
    keys: Vec<String>,
}

impl RedisCommandMGet {
    pub fn new(keys: Vec<String>) -> RedisCommandMGet {
        RedisCommandMGet { keys }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command MGet with keys: {:?} ",
            self.keys
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::MGet(self.keys.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        verbose.print("Finalizing execution of command MGet");
        Ok(response)
    }
}

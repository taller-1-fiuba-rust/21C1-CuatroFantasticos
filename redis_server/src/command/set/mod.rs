use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSet {
    key: String,
    new_value: String,
}

impl RedisCommandSet {
    pub fn new(key: String, new_value: String) -> RedisCommandSet {
        RedisCommandSet { key, new_value }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Set with key: {} and new_value : {} ",
            self.key, self.new_value
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Set(self.key.clone(), self.new_value.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        verbose.print("Finalizing execution of command Set");
        Ok(response)
    }
}

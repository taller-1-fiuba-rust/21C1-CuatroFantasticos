use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

/// Saves an element in Storage

pub struct RedisCommandSave {}

impl RedisCommandSave {
    pub fn new() -> RedisCommandSave {
        RedisCommandSave {}
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print("Executing command Save");
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Save)?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        verbose.print("Finalizing execution of command Save");
        Ok(response)
    }
}

impl Default for RedisCommandSave {
    fn default() -> Self {
        Self::new()
    }
}

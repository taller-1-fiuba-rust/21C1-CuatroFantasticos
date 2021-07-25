use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLPush {
    key: String,
    values: Vec<String>,
}

impl RedisCommandLPush {
    pub fn new(key: String, values: Vec<String>) -> RedisCommandLPush {
        RedisCommandLPush { key, values }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command LPush with key: {} and values: {:?}",
            self.key, self.values
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::LPush(self.key.clone(), self.values.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of commmand LPush");
        Ok(response)
    }
}

use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the string representation of the type of the value stored at key.
/// The different types that can be returned are: string, list, set, zset, hash and stream.
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Simple string reply: type of key, or none when key does not exist.

pub struct RedisCommandType {
    key: String,
}

impl RedisCommandType {
    pub fn new(key: String) -> RedisCommandType {
        RedisCommandType { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!("Executing command Type with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Type(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(value) => {
                value.get_type().protocol_serialize_to_simple_string()
            }
            value => value.protocol_serialize_to_simple_string(),
        };
        verbose.print("Finalizing execution of command Type");
        Ok(response)
    }
}

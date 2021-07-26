use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the length of the string value stored at key. An error is returned when key holds a non-string value.
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Integer reply: the length of the string at key, or 0 when key does not exist.

pub struct RedisCommandStrlen {
    key: String,
}

impl RedisCommandStrlen {
    pub fn new(key: String) -> RedisCommandStrlen {
        RedisCommandStrlen { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!("Executing command Strlen with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Strlen(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Strlen");
        Ok(response)
    }
}

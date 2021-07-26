use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the length of the list stored at key.
/// If key does not exist, it is interpreted as an empty list and 0 is returned.
/// An error is returned when the value stored at key is not a list.
///
/// # Arguments
/// * key - String
///
/// # Return value
///Integer reply: the length of the list at key.

pub struct RedisCommandLlen {
    key: String,
}

impl RedisCommandLlen {
    pub fn new(key: String) -> RedisCommandLlen {
        RedisCommandLlen { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!("Executing command Llen with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Llen(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Llen");
        Ok(response)
    }
}

use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Get the value of key and delete the key. This command is similar to GET,
/// except for the fact that it also deletes the key on success (if and only if the key's value type is a string).
///
/// # Arguments
/// * key - String
///
/// # Return value
///Bulk string reply: the value of key, nil when key does not exist,
/// or an error if the key's value type isn't a string.

pub struct RedisCommandGetDel {
    key: String,
}

impl RedisCommandGetDel {
    pub fn new(key: String) -> RedisCommandGetDel {
        RedisCommandGetDel { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!("Executing command GetDel with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::GetDel(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        verbose.print("Finalizing execution of command GetDel");
        Ok(response)
    }
}

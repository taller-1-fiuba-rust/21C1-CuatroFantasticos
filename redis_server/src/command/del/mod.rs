use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Removes the specified keys. A key is ignored if it does not exist.
/// # Arguments
///  * key - String
///
/// # Return value
///Integer reply: The number of keys that were removed.

pub struct RedisCommandDel {
    key: String,
}

impl RedisCommandDel {
    pub fn new(key: String) -> RedisCommandDel {
        RedisCommandDel { key }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!("Executing command Del with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Del(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Del");
        Ok(response)
    }
}

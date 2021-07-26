use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Alters the last access time of a key(s). A key is ignored if it does not exist.
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Integer reply: The number of keys that were touched.

pub struct RedisCommandTouch {
    key: String,
}

impl RedisCommandTouch {
    pub fn new(key: String) -> RedisCommandTouch {
        RedisCommandTouch { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!("Executing command Touch with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Touch(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Touch");
        Ok(response)
    }
}

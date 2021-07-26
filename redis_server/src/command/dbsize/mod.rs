use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Return the number of keys in the currently-selected database.
/// # Return value
///* Integer reply

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print("Executing command DbSize");
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Dbsize)?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of comand Copy");
        Ok(response)
    }
}

impl Default for RedisCommandDbSize {
    fn default() -> Self {
        Self::new()
    }
}

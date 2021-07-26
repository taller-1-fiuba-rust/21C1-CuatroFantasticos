use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///If key already exists and is a string, this command appends the value at the end of the string.
/// If key does not exist it is created and set as an empty string, so APPEND will be similar to SET
/// in this special case.
///
/// # Arguments
/// * key - String
/// * new_value - String
///
/// # Return value
///Integer reply: the length of the string after the append operation.

pub struct RedisCommandAppend {
    key: String,
    new_value: String,
}

impl RedisCommandAppend {
    pub fn new(key: String, new_value: String) -> RedisCommandAppend {
        RedisCommandAppend { key, new_value }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Append with key: {} and new_value: {}",
            self.key, self.new_value
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Append(
                self.key.clone(),
                self.new_value.clone(),
            ))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Append");
        Ok(response)
    }
}

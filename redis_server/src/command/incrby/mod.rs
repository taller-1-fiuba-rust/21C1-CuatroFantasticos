use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Increments the number stored at key by decrement.
/// If the key does not exist, it is set to 0 before performing the operation.
/// An error is returned if the key contains a value of the wrong type
/// or contains a string that can not be represented as integer.
/// This operation is limited to 64 bit signed integers.
/// # Arguments
/// * key - String
/// * new_value - String
///
/// # Return value
///Integer reply: the value of key after the increment

pub struct RedisCommandIncrBy {
    key: String,
    new_value: String,
}

impl RedisCommandIncrBy {
    pub fn new(key: String, new_value: String) -> RedisCommandIncrBy {
        RedisCommandIncrBy { key, new_value }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command IncrBy with key: {} and value: {}",
            self.key, self.new_value
        ));
        let value = self.new_value.parse::<i32>();
        let response = match value {
            Ok(value) => {
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::IncrBy(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => RedisError::NotANumber.protocol_serialize_to_bulk_string(),
        };
        verbose.print("Finalizing execution of command Incrby");
        Ok(response)
    }
}

use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Decrements the number stored at key by decrement.
/// If the key does not exist, it is set to 0 before performing the operation.
/// An error is returned if the key contains a value of the wrong type
/// or contains a string that can not be represented as integer.
/// This operation is limited to 64 bit signed integers.
/// # Arguments
/// * key - String
/// * new_value - String
///
/// # Return value
///Integer reply: the value of key after the decrement

pub struct RedisCommandDecrBy {
    key: String,
    new_value: String,
}

impl RedisCommandDecrBy {
    pub fn new(key: String, new_value: String) -> RedisCommandDecrBy {
        RedisCommandDecrBy { key, new_value }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command DecrBy with key: {} and new_value: {}",
            self.key, self.new_value
        ));
        let value = self.new_value.parse::<i32>();
        let response = match value {
            Ok(value) => {
                verbose.print(&format!(
                    "Value of argument was an integer of value: {}",
                    value
                ));
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::DecrBy(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => {
                verbose.print("Error, value of argument was not an integer");
                RedisError::NotANumber.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command DecrBy");
        Ok(response)
    }
}

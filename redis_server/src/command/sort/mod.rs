use crate::data::redis_value::RedisValue;

use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

/// Returns or stores the elements contained in the list,
/// set or sorted set at key. By default, sorting is numeric
/// and elements are compared by their value interpreted as double
/// precision floating point number.
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Array reply: without passing the store option the command returns a list of sorted elements.

pub struct RedisCommandSort {
    key: String,
}

impl RedisCommandSort {
    pub fn new(key: String) -> RedisCommandSort {
        RedisCommandSort { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!("Executing command Sort with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Get(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(RedisValue::Set(value)) => match value.sort() {
                Ok(value) => {
                    verbose.print("Sorted redisValue Set successfully");
                    value.protocol_serialize_to_bulk_string()
                }
                Err(value) => {
                    verbose.print("Could not sort redisValue Set successfully");
                    value.protocol_serialize_to_bulk_string()
                }
            },
            StorageResult::RedisValue(RedisValue::List(value)) => match value.sort() {
                Ok(value) => {
                    verbose.print("Sorted redisValue List successfully");
                    value.protocol_serialize_to_bulk_string()
                }
                Err(value) => {
                    verbose.print("Could not sort redisValue List successfully");
                    value.protocol_serialize_to_bulk_string()
                }
            },
            StorageResult::RedisValue(RedisValue::String(_)) => {
                verbose.print("Got a redisValue String response");
                RedisError::NotAListNorSet.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(RedisError::NonExistent) => {
                verbose.print("Got a NonExistent Error response");
                RedisError::NilArray.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(value) => {
                verbose.print("Got an Error response");
                value.protocol_serialize_to_bulk_string()
            }
            _ => {
                verbose.print("Unexpected error response");
                StorageResult::Error(RedisError::Unknown).protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command Sort");
        Ok(response)
    }
}

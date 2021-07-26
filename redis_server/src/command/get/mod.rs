use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Get the value of key. If the key does not exist the special value nil is returned.
/// An error is returned if the value stored at key is not a string, because GET only handles string values.
///
/// # Arguments
/// *  key - String
///
/// # Return value
///Bulk string reply: the value of key, or nil when key does not exist.

pub struct RedisCommandGet {
    key: String,
}

impl RedisCommandGet {
    pub fn new(key: String) -> RedisCommandGet {
        RedisCommandGet { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!("Executing command Get with key: {}", self.key));

        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Get(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(RedisValue::String(value)) => {
                verbose.print("Got a RedisValue String response");
                value.protocol_serialize_to_bulk_string()
            }
            StorageResult::RedisValue(_) => {
                verbose.print("Error, the response was either a Set or a List");
                RedisError::NotAString.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(value) => {
                verbose.print("Error, got an Error response");
                value.protocol_serialize_to_bulk_string()
            }
            _ => {
                verbose.print("Error, something unexpected happened");
                RedisError::Unknown.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command Get");
        Ok(response)
    }
}

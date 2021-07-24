use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::protocol_serialization::ProtocolSerializer;
///Returns the string representation of the type of the value stored at key.
/// The different types that can be returned are: string, list, set, zset, hash and stream.
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Simple string reply: type of key, or none when key does not exist.

pub struct RedisCommandType {
    key: String,
}

impl RedisCommandType {
    pub fn new(key: String) -> RedisCommandType {
        RedisCommandType { key }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Type(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(value) => {
                value.get_type().protocol_serialize_to_simple_string()
            }
            value => value.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}

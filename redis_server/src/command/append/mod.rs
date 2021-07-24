use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
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
}

impl RedisCommand for RedisCommandAppend {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Append(
            self.key.clone(),
            self.new_value.clone(),
        ))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
